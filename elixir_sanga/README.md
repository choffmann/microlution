# ElixirSanga

Elixir library for communicating with the Sanga motor controller board via serial port.

## Communication Modes

This library supports two different communication modes:

### 1. Persistent Connection Mode (Fast, Exclusive Access)
Maintains a persistent connection to the serial port for high-frequency communication. This is faster but holds an exclusive lock on the serial port.

```elixir
# Start the persistent connection
{:ok, _pid} = Sanga.Board.start_link()

# Send commands
{:ok, version} = Sanga.Board.query("v")
{:ok, result} = Sanga.Board.multi_query(["n_motors 4", "mr 0 0 0 100", "n_motors 3"])
{:ok, result} = Sanga.Board.move_slider(100)

# Stop the connection
:ok = Sanga.Board.stop()
```

### 2. Safe Connection Mode (Slower, Shared Access)
Opens and closes the connection for each operation, allowing other services to access the serial port. This is slower due to connection overhead but allows multiple services to communicate with the motor controller.

```elixir
# No need to start/stop - each command manages its own connection
{:ok, version} = Sanga.Board.safe_query("v")
{:ok, result} = Sanga.Board.safe_multi_query(["n_motors 4", "mr 0 0 0 100", "n_motors 3"])
{:ok, result} = Sanga.Board.safe_move_slider(100)

# Stage control methods
{:ok, result} = Sanga.Board.safe_move_stage_x(10.5)    # Move stage in X direction
{:ok, result} = Sanga.Board.safe_move_stage_y(-5.2)    # Move stage in Y direction  
{:ok, result} = Sanga.Board.safe_move_stage_z(2.0)     # Move stage in Z direction
{:ok, result} = Sanga.Board.safe_release_stage()       # Release motor holding torque
{:ok, result} = Sanga.Board.safe_zero_stage()          # Zero/home the stage
```

### When to Use Each Mode

- **Persistent Mode**: Use when you need exclusive access and high-frequency communication
- **Safe Mode**: Use when other services also need to communicate with the motor controller

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `elixir_sanga` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:elixir_sanga, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/elixir_sanga>.

