# Test script for safe board communication
# This demonstrates using the safe_query methods that don't hold a persistent connection

# Test safe query methods
IO.puts("=== Testing Safe Board Communication ===")
IO.puts("These methods open/close the connection for each command")
IO.puts("This allows other services to also access the motor controller\n")

# Test single safe query
IO.puts("1. Testing safe_query:")
case Sanga.Board.safe_query("version") do
  {:ok, version} ->
    IO.puts("   Motor controller version: #{version}")
  {:error, reason} ->
    IO.puts("   Error: #{inspect(reason)}")
end

# Test safe multi-query
IO.puts("\n2. Testing safe_multi_query:")
commands = ["n_motors 4", "mr 10 10 0 500", "n_motors 3"]
case Sanga.Board.safe_multi_query(commands) do
  {:ok, result} ->
    IO.puts("   Multi-query result: #{result}")
  {:error, reason} ->
    IO.puts("   Error: #{inspect(reason)}")
end

# Test safe move slider
IO.puts("\n3. Testing safe_move_slider:")
case Sanga.Board.safe_move_slider(100) do
  {:ok, result} ->
    IO.puts("   Slider moved successfully: #{result}")
  {:error, reason} ->
    IO.puts("   Error: #{inspect(reason)}")
end

# Test stage control methods
IO.puts("\n4. Testing stage control methods:")

IO.puts("   - Moving stage in X direction:")
case Sanga.Board.safe_move_stage_x(10) do
  {:ok, result} ->
    IO.puts("     Stage X movement: #{result}")
  {:error, reason} ->
    IO.puts("     Error: #{inspect(reason)}")
end

IO.puts("   - Moving stage in Y direction:")
case Sanga.Board.safe_move_stage_y(5) do
  {:ok, result} ->
    IO.puts("     Stage Y movement: #{result}")
  {:error, reason} ->
    IO.puts("     Error: #{inspect(reason)}")
end

IO.puts("   - Moving stage in Z direction:")
case Sanga.Board.safe_move_stage_z(-2) do
  {:ok, result} ->
    IO.puts("     Stage Z movement: #{result}")
  {:error, reason} ->
    IO.puts("     Error: #{inspect(reason)}")
end

IO.puts("   - Releasing stage motors:")
case Sanga.Board.safe_release_stage() do
  {:ok, result} ->
    IO.puts("     Stage released: #{result}")
  {:error, reason} ->
    IO.puts("     Error: #{inspect(reason)}")
end
