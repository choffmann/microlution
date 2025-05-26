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

  def query(command) do
    GenServer.call(__MODULE__, {:query, command}, @response_timeout + 500)
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

  @impl true
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

  def handle_info({:circuits_uart, _port, data}, state) do
    # No pending query, just log the data
    IO.puts("Unexpected Sanga response: #{inspect(data)}")
    {:noreply, state}
  end

  def handle_info(:query_timeout, %{current_caller: caller} = state) when not is_nil(caller) do
    # Reply with timeout error
    GenServer.reply(caller, {:error, :timeout})

    # Clear current query state
    new_state = %{state | current_query: nil, current_caller: nil}
    {:noreply, new_state}
  end

  def handle_info(:query_timeout, state) do
    # No active query or already handled
    {:noreply, state}
  end

  def handle_info({:circuits_uart, _port, {:error, reason}}, state) do
    IO.puts("! Serial error: #{reason}")
    {:noreply, state}
  end
end
