defmodule Sanga.Board do
  @moduledoc """
  GenServer for communicating with the Sanga motor controller board via serial port.

  This module provides two communication modes:

  ## Persistent Connection Mode
  Maintains a persistent connection for high-frequency communication:
  - `start_link/0` - Start the GenServer with persistent connection
  - `query/1` - Send a single command using persistent connection
  - `multi_query/1` - Send multiple commands using persistent connection
  - `move_slider/1` - Move slider using persistent connection
  - `stop/0` - Stop the GenServer and close connection

  ## Safe Connection Mode
  Opens/closes connection for each operation, allowing shared access:
  - `safe_query/1` - Send a single command with temporary connection
  - `safe_multi_query/1` - Send multiple commands with temporary connections
  - `safe_move_slider/1` - Move slider with temporary connections
  - `safe_move_stage_x/1` - Move stage in X direction
  - `safe_move_stage_y/1` - Move stage in Y direction
  - `safe_move_stage_z/1` - Move stage in Z direction
  - `safe_release_stage/0` - Release stage motors
  - `safe_zero_stage/0` - Zero/home the stage
  - `port_available?/0` - Check if serial port is available

  Use persistent mode for exclusive access and high performance.
  Use safe mode when other services also need to access the motor controller.
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
    framing: {Circuits.UART.Framing.Line, separator: "\n"}
  ]
  # Timeout for waiting for responses (in ms)
  @response_timeout 100_000

  # Public API

  # === PERSISTENT CONNECTION METHODS ===
  # These methods maintain a persistent connection to the serial port for high-frequency communication.
  # Faster but holds an exclusive lock on the serial port.
  # Use when you need exclusive access and high performance.

  def start_link() do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
  end

  def stop() do
    GenServer.call(__MODULE__, :stop)
  end

  def query(command) do
    GenServer.call(__MODULE__, {:query, command}, @response_timeout + 500)
  end

  # === SAFE CONNECTION METHODS ===
  # These methods open/close the connection for each operation, allowing other services
  # to access the serial port. Slower due to connection overhead but plays nicely with
  # other software that needs to communicate with the motor controller.
  # Safe query that opens/closes connection for each command - allows other services to access the port
  def safe_query(command) do
    pid = nil

    try do
      with {:ok, uart_pid} <- Circuits.UART.start_link(),
           :ok <- Circuits.UART.open(uart_pid, @default_port, @serial_settings),
           :ok <- Circuits.UART.write(uart_pid, command) do

        pid = uart_pid

        # Wait for response with timeout
        receive do
          {:circuits_uart, ^uart_pid, data} ->
            {:ok, String.trim(data)}

          {:circuits_uart, ^uart_pid, {:error, reason}} ->
            {:error, reason}
        after
          @response_timeout ->
            {:error, :timeout}
        end      else
        error ->
          {:error, "Failed to open serial connection: #{inspect(error)}"}
      end
    after
      # Always clean up the UART connection, regardless of success or failure
      if pid do
        cleanup_uart_connection(pid)
      end
    end
  end

  # Helper function to ensure proper cleanup of UART connection
  defp cleanup_uart_connection(pid) do
    try do
      # Close the serial port first
      case Circuits.UART.close(pid) do
        :ok -> :ok
        {:error, :not_open} -> :ok  # Already closed
        error ->
          IO.puts("Warning: Failed to close UART: #{inspect(error)}")
      end

      # Stop the UART process
      case Circuits.UART.stop(pid) do
        :ok -> :ok
        error ->
          IO.puts("Warning: Failed to stop UART process: #{inspect(error)}")
      end

      # Give the system a moment to fully release the port
      Process.sleep(5)
    catch
      kind, reason ->
        IO.puts("Warning: Exception during UART cleanup: #{kind} - #{inspect(reason)}")
    end
  end

  # Safe version of multi_query using safe_query
  def safe_multi_query(commands) when is_list(commands) and length(commands) > 0 do
    execute_safe_queries(commands)
  end

  # sets n_motors back to default value so that the board can be used with other software
  def move_slider(value) when is_integer(value) do
    multi_query([
      "n_motors 4",
      "mr 0 0 0 #{value}",
      "n_motors 3"
    ])
  end

  # Safe version of move_slider
  def safe_move_slider(value) when is_integer(value) do
    safe_multi_query([
      "n_motors 4",
      "mr 0 0 0 #{value}",
      "n_motors 3"
    ])
  end

  # === STAGE CONTROL METHODS (SAFE) ===
  # These methods control the stage movement and status

  # Move stage in X direction
  def safe_move_stage_x(distance) when is_number(distance) do
    safe_query("mrx #{distance}")
  end

  # Move stage in Y direction
  def safe_move_stage_y(distance) when is_number(distance) do
    safe_query("mry #{distance}")
  end

  # Move stage in Z direction
  def safe_move_stage_z(distance) when is_number(distance) do
    safe_query("mrz #{distance}")
  end
  # Release stage motors (disable holding torque)
  def safe_release_stage() do
    safe_query("release")
  end

  # Zero/home the stage
  def safe_zero_stage() do
    safe_query("zero")
  end

  def multi_query(commands) when is_list(commands) and length(commands) > 0 do
    execute_queries(commands, nil)
  end
  defp execute_queries([command | rest], _) do
    case query(command) do
      {:ok, result} ->
        if rest == [] do
          # This is the last query, return its result
          {:ok, result}
        else
          # Continue to the next query
          execute_queries(rest, {:ok, result})
        end

      error ->
        # If any query fails, stop and return the error
        error
    end
  end

  defp execute_safe_queries([command | rest]) do
    case safe_query(command) do
      {:ok, result} ->
        if rest == [] do
          # This is the last query, return its result
          {:ok, result}
        else
          # Continue to the next query
          execute_safe_queries(rest)
        end

      error ->
        # If any query fails, stop and return the error
        error
    end
  end
  # Utility function to check if the serial port is available
  def port_available?() do
    case Circuits.UART.start_link() do
      {:ok, pid} ->
        case Circuits.UART.open(pid, @default_port, @serial_settings) do
          :ok ->
            cleanup_uart_connection(pid)
            true
          error ->
            cleanup_uart_connection(pid)
            IO.puts("Port unavailable: #{inspect(error)}")
            false
        end
      error ->
        IO.puts("Failed to start UART: #{inspect(error)}")
        false
    end
  end

  # GenServer Implementation
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
    # The framing will automatically add the separator
    :ok = Circuits.UART.write(state.uart_pid, query)

    # Store the current query and caller
    new_state = %{state | current_query: query, current_caller: from}

    # Set a timeout to avoid hanging if no response comes
    Process.send_after(self(), :query_timeout, @response_timeout)

    {:noreply, new_state}
  end

  def handle_call({:query, _query}, _from, state) do
    # Another query is already in progress
    {:reply, {:error, :busy}, state}
  end

  def handle_call(:stop, _from, state) do
    if state.uart_pid != nil do
      # Close the serial port
      :ok = Circuits.UART.close(state.uart_pid)
      # Stop the UART GenServer process
      :ok = Circuits.UART.stop(state.uart_pid)

      IO.puts("Sanga board connection closed")
    end

    # Return success and clear the state
    {:reply, :ok, %{state | uart_pid: nil, current_query: nil, current_caller: nil}}
  end

  @impl true
  # Handle UART errors
  def handle_info({:circuits_uart, _port, {:error, reason}}, state) do
    IO.puts("! Serial error: #{inspect(reason)}")
    {:noreply, state}
  end

  # Handle responses when there's an active caller
  def handle_info({:circuits_uart, _port, data}, %{current_caller: caller} = state)
      when not is_nil(caller) do
    # Log the response for debugging
    IO.puts("Sanga response: #{inspect(data)}")

    # Reply to the caller
    GenServer.reply(caller, {:ok, data})

    # Clear current query state
    new_state = %{state | current_query: nil, current_caller: nil}
    {:noreply, new_state}
  end

  # Handle unexpected responses (no active query)
  def handle_info({:circuits_uart, _port, data}, state) do
    # No pending query, just log the data
    IO.puts("Unexpected Sanga response: #{inspect(data)}")
    {:noreply, state}
  end

  # Handle query timeouts for active queries
  def handle_info(:query_timeout, %{current_caller: caller} = state) when not is_nil(caller) do
    # Reply with timeout error
    GenServer.reply(caller, {:error, :timeout})

    # Clear current query state
    new_state = %{state | current_query: nil, current_caller: nil}
    {:noreply, new_state}
  end

  # Handle orphaned timeout messages
  def handle_info(:query_timeout, state) do
    # No active query or already handled
    {:noreply, state}
  end
end
