defmodule Sanga.Board do
  @moduledoc """
  GenServer for communicating with the Sanga motor controller board via serial port.

  Provides two communication modes:
  - Persistent: High-frequency, exclusive access.
  - Safe: Opens/closes connection per operation for shared access.
  """

  use GenServer

  # Hardware defaults
  @default_port "ttyAMA0"
  @default_baud 115_200
  @serial_settings [
    speed: @default_baud,
    data_bits: 8,
    parity: :none,
    stop_bits: 1,
    active: true,
    framing: {Circuits.UART.Framing.Line, separator: "\r\n"}
  ]
  @response_timeout 100_000

  # === PERSISTENT CONNECTION METHODS ===

  @doc "Start the GenServer with a persistent UART connection."
  def start_link(), do: GenServer.start_link(__MODULE__, [], name: __MODULE__)

  @doc "Stop the GenServer and close the UART connection."
  def stop(), do: GenServer.call(__MODULE__, :stop)

  @doc "Send a command using the persistent connection."
  def query(command), do: GenServer.call(__MODULE__, {:query, command}, @response_timeout + 500)

  @doc "Send multiple commands using the persistent connection."
  def multi_query(commands) when is_list(commands) and commands != [],
    do: do_multi_query(commands, &query/1)

  # === SAFE CONNECTION METHODS ===

  @doc "Send a single command with a temporary UART connection."
  def safe_query(command) do
    with {:ok, uart_pid} <- Circuits.UART.start_link(),
         :ok <- Circuits.UART.open(uart_pid, @default_port, @serial_settings),
         result <- do_safe_query(uart_pid, command) do
      cleanup_uart_connection(uart_pid)
      result
    else
      error -> error
    end
  end

  @doc "Send multiple commands with temporary UART connections."
  def safe_multi_query(commands) when is_list(commands) and commands != [],
    do: do_multi_query(commands, &safe_query/1)

  @doc "Move slider with temporary UART connections."
  def safe_move_slider(value) when is_integer(value),
    do: safe_multi_query(["n_motors 4", "mr 0 0 0 #{value}", "n_motors 3"])

  @doc "Move stage in X direction."
  def safe_move_stage_x(distance) when is_number(distance),
    do: safe_query("mrx #{distance}")

  @doc "Move stage in Y direction."
  def safe_move_stage_y(distance) when is_number(distance),
    do: safe_query("mry #{distance}")

  @doc "Move stage in Z direction."
  def safe_move_stage_z(distance) when is_number(distance),
    do: safe_query("mrz #{distance}")

  @doc "Release stage motors."
  def safe_release_stage(), do: safe_query("release")

  @doc "Zero/home the stage."
  def safe_zero_stage(), do: safe_query("zero")

  @doc "Check if the serial port is available."
  def port_available?() do
    with {:ok, pid} <- Circuits.UART.start_link(),
         :ok <- Circuits.UART.open(pid, @default_port, @serial_settings) do
      cleanup_uart_connection(pid)
      true
    else
      error ->
        IO.puts("Port unavailable: #{inspect(error)}")
        false
    end
  end

  # Legacy compatibility method for existing code
  @doc "Move slider (legacy method for backward compatibility)."
  def move_slider(value) when is_integer(value),
    do: multi_query(["n_motors 4", "mr 0 0 0 #{value}", "n_motors 3"])

  # === PRIVATE HELPERS ===

  defp do_safe_query(uart_pid, command) do
    :ok = Circuits.UART.write(uart_pid, command)

    receive do
      {:circuits_uart, _port, data} ->
        {:ok, String.trim(data)}

      other ->
        IO.inspect(other, label: "Received unexpected UART message")
        {:error, :unexpected_message}
    after
      @response_timeout ->
        {:error, :timeout}
    end
  end

  defp cleanup_uart_connection(pid) do
    try do
      Circuits.UART.close(pid)
      Circuits.UART.stop(pid)
      Process.sleep(5)
    catch
      kind, reason ->
        IO.puts("Warning: Exception during UART cleanup: #{kind} - #{inspect(reason)}")
    end
  end

  defp do_multi_query([command], fun), do: fun.(command)

  defp do_multi_query([command | rest], fun) do
    case fun.(command) do
      {:ok, _} -> do_multi_query(rest, fun)
      error -> error
    end
  end

  # === GEN_SERVER CALLBACKS ===

  @impl true
  def init(_opts) do
    with {:ok, pid} <- Circuits.UART.start_link(),
         :ok <- Circuits.UART.open(pid, @default_port, @serial_settings) do
      {:ok, %{uart_pid: pid, current_query: nil, current_caller: nil}}
    else
      error -> {:stop, "Failed to initialize serial port: #{inspect(error)}"}
    end
  end

  @impl true
  def handle_call({:query, query}, from, %{current_query: nil} = state) do
    :ok = Circuits.UART.write(state.uart_pid, query)
    new_state = %{state | current_query: query, current_caller: from}
    Process.send_after(self(), :query_timeout, @response_timeout)
    {:noreply, new_state}
  end

  def handle_call({:query, _query}, _from, state),
    do: {:reply, {:error, :busy}, state}

  def handle_call(:stop, _from, state) do
    if state.uart_pid do
      Circuits.UART.close(state.uart_pid)
      Circuits.UART.stop(state.uart_pid)
      IO.puts("Sanga board connection closed")
    end

    {:reply, :ok, %{state | uart_pid: nil, current_query: nil, current_caller: nil}}
  end

  @impl true
  def handle_info({:circuits_uart, _port, {:error, reason}}, state) do
    IO.puts("! Serial error: #{inspect(reason)}")
    {:noreply, state}
  end

  def handle_info({:circuits_uart, _port, data}, %{current_caller: caller} = state)
      when not is_nil(caller) do
    IO.puts("Sanga response: #{inspect(data)}")
    GenServer.reply(caller, {:ok, data})
    {:noreply, %{state | current_query: nil, current_caller: nil}}
  end

  def handle_info({:circuits_uart, _port, data}, state) do
    IO.puts("Unexpected Sanga response: #{inspect(data)}")
    {:noreply, state}
  end

  def handle_info(:query_timeout, %{current_caller: caller} = state) when not is_nil(caller) do
    GenServer.reply(caller, {:error, :timeout})
    {:noreply, %{state | current_query: nil, current_caller: nil}}
  end

  def handle_info(:query_timeout, state), do: {:noreply, state}
end
