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
Process.sleep(500)
# Execute three queries in sequence, only getting the final result
case Sanga.Board.multi_query("n_motors 4", "mr 0 0 0 5000", "mr 0 0 0 -5000", "n_motors 3") do
  {:ok, result} -> IO.puts("Got final result: #{result}")
  {:error, reason} -> IO.puts("Error: #{reason}")
end
# Give some time for the UART to finish sending data
Process.sleep(500)
