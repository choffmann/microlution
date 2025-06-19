defmodule Server.Navigation do
  alias Server.Settings
  alias Server.Api

  def move_in_direction(direction, step_size) do
    settings = Settings.get_settings!(1)

    move_in_direction =
      get_navigate_direction(direction, step_size)

    Phoenix.PubSub.broadcast(
      Server.PubSub,
      "update-minimap",
      {:update_minimap, direction, step_size}
    )

    Settings.update(1, %{
      "current_x" => settings.current_x + move_in_direction.x
    })

    Settings.update(1, %{
      "current_y" => settings.current_y + move_in_direction.y
    })

    case HTTPoison.post(
           Api.move_to_in_image_coords(),
           move_in_direction |> Jason.encode!(),
           [{"Content-Type", "application/json"}]
         ) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body

      _ ->
        []
    end
  end

  def move_to_home() do
    settings = Settings.get_settings!(1)

    move_in_direction =
      %{x: settings.current_x * -1, y: settings.current_y * -1}

    Settings.update(1, %{
      "current_x" => 0
    })

    settings =
      Settings.update(1, %{
        "current_y" => 0
      })

    case HTTPoison.post(
           Api.move_to_in_image_coords(),
           move_in_direction |> Jason.encode!(),
           [{"Content-Type", "application/json"}]
         ) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body

      _ ->
        []
    end
  end

  def set_home() do
    settings = Settings.get_settings!(1)

    Settings.update(1, %{
      "current_x" => 0
    })

    Settings.update(1, %{
      "current_y" => 0
    })
  end

  def get_navigate_direction(direction, step_size) do
    case direction do
      "up-left" ->
        %{x: -step_size, y: step_size}

      "up" ->
        %{x: -step_size, y: 0}

      "up-right" ->
        %{x: -step_size, y: -step_size}

      "down-left" ->
        %{x: step_size, y: step_size}

      "down" ->
        %{x: step_size, y: 0}

      "down-right" ->
        %{x: step_size, y: -step_size}

      "left" ->
        %{x: 0, y: step_size}

      "right" ->
        %{x: 0, y: -step_size}
    end
  end

  def get_navigate_direction_minimap(direction, step_size) do
    case direction do
      "up-left" ->
        %{x: -step_size, y: -step_size}

      "up" ->
        %{x: 0, y: -step_size}

      "up-right" ->
        %{x: step_size, y: -step_size}

      "down-left" ->
        %{x: -step_size, y: step_size}

      "down" ->
        %{x: 0, y: step_size}

      "down-right" ->
        %{x: step_size, y: step_size}

      "left" ->
        %{x: -step_size, y: 0}

      "right" ->
        %{x: step_size, y: 0}
    end
  end
end
