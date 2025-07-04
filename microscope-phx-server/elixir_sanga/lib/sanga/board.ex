defmodule Sanga.Board do
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
  def start_link() do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
  end

  def stop() do
    GenServer.call(__MODULE__, :stop)
  end

  def query(command) do
    GenServer.call(__MODULE__, {:query, command}, @response_timeout + 500)
  end

  # sets n_motors back to default value so that the board can be used with other software
  def move_slider(value) when is_integer(value) do
    multi_query([
      "n_motors 4",
      "mr 0 0 0 #{value}",
      "n_motors 3"
    ])
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
