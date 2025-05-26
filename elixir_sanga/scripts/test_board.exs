# Simple script to test Sanga board connectivity

# Start required applications
Application.ensure_all_started(:circuits_uart)

# Start the board interface
{:ok, _pid} = Sanga.Board.start_link()

IO.puts("Sending version query to Sanga board...")

# Send the version query
case Sanga.Board.query("version") do
  {:ok, response} ->
    IO.puts("Success! Response: #{response}")

  {:error, :timeout} ->
    IO.puts("Error: Query timed out - check if the board is connected and responding")

  {:error, reason} ->
    IO.puts("Error: #{inspect(reason)}")
end

# Give some time for the UART to finish sending data
Process.sleep(500)
