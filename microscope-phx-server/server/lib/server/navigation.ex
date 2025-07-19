defmodule Server.Navigation do
  alias Server.Settings
  alias Server.Api
  alias Sanga.Board

  def move_in_direction(direction, step_size) do
    settings = Settings.get_settings!(1)

    move_in_direction =
      cond do
        settings.navigation_minimap ->
          get_navigate_direction(direction, step_size)

        !settings.navigation_minimap ->
          get_navigate_direction_sanga(direction, step_size)
      end

    Settings.update(1, %{
      "current_x" => settings.current_x + move_in_direction.x
    })

    Settings.update(1, %{
      "current_y" => settings.current_y + move_in_direction.y
    })

    move_sanga(move_in_direction)

    update_minimap(direction, step_size)
  end

  def update_minimap(direction, step_size) do
    settings = Settings.get_settings!(1)

    move_in_direction =
      get_navigate_direction_minimap(direction, step_size, settings.navigation_minimap)
      |> IO.inspect()

    boundaries = %{boundaryx: settings.boundary_x, boundaryy: settings.boundary_y}

    Settings.update(1, %{
      "minimap_x" => settings.minimap_x + move_in_direction.x,
      "minimap_y" => settings.minimap_y + move_in_direction.y
    })

    Map.merge(move_in_direction, boundaries)
    |> Map.merge(%{show_mm_features: settings.show_mm_features})
    |> IO.inspect()
  end

  def refresh_minimap() do
    update_minimap("up", 0)
  end

  def move_stage(direction, step_size) do
    settings = Settings.get_settings!(1)

    no_minimap_update = %{
      boundaryx: settings.boundary_x,
      boundaryy: settings.boundary_y,
      x: 0,
      y: 0
    }

    move_in_direction =
      cond do
        settings.navigation_minimap ->
          get_navigate_direction(direction, step_size)

        !settings.navigation_minimap ->
          get_navigate_direction_sanga(direction, step_size)
      end

    if move_in_direction.y > 0 or move_in_direction.x > 0 do
      if settings.current_y + move_in_direction.y > settings.boundary_y or
           settings.current_x + move_in_direction.x > settings.boundary_x do
        IO.inspect("Boundary Positive X")
        {no_minimap_update, :error, ""}
      else
        if settings.boundary_y - settings.current_y >= move_in_direction.y or
             settings.boundary_x - settings.current_x >= move_in_direction.x do
          update_minimap = move_in_direction(direction, step_size)
          {update_minimap, :info, ""}
        end
      end
    else
      if settings.current_y <= -settings.boundary_y or settings.current_x <= -settings.boundary_x do
        IO.inspect("Boundary Negative X")
        {no_minimap_update, :error, ""}
      else
        if -settings.boundary_y <= settings.current_y + move_in_direction.y or
             -settings.boundary_x <= settings.current_x + move_in_direction.x do
          update_minimap = move_in_direction(direction, step_size)
          {update_minimap, :info, ""}
        end
      end
    end
  end

  def move_to_home() do
    settings = Settings.get_settings!(1)

    move_in_direction =
      %{x: settings.current_x * -1, y: settings.current_y * -1}

    Settings.update(1, %{
      "current_x" => 0,
      "current_y" => 0,
      "minimap_x" => 0,
      "minimap_y" => 0
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

  def get_navigate_direction_sanga(direction, step_size) do
    case direction do
      "up-left" ->
        %{x: step_size, y: step_size}

      "up" ->
        %{x: 0, y: step_size}

      "up-right" ->
        %{x: -step_size, y: step_size}

      "down-left" ->
        %{x: step_size, y: -step_size}

      "down" ->
        %{x: 0, y: -step_size}

      "down-right" ->
        %{x: -step_size, y: -step_size}

      "left" ->
        %{x: step_size, y: 0}

      "right" ->
        %{x: -step_size, y: 0}
    end
  end

  def get_navigate_direction(direction, step_size) do
    case direction do
      "up" ->
        %{x: step_size, y: step_size}

      "down" ->
        %{x: -step_size, y: -step_size}

      "left" ->
        %{x: -step_size, y: step_size}

      "right" ->
        %{x: step_size, y: -step_size}

      "up-left" ->
        %{x: 0, y: step_size}

      "up-right" ->
        %{x: step_size, y: 0}

      "down-left" ->
        %{x: -step_size, y: 0}

      "down-right" ->
        %{x: 0, y: -step_size}
    end
  end

  def move_sanga(direction) do
    Sanga.Board.safe_move_all_axes(0, 0, 0, 0)
    Sanga.Board.safe_move_all_axes(direction.x, direction.y, 0, 0)
  end

  def get_navigate_direction_minimap(direction, step_size, navigation_minimap) do
    if navigation_minimap do
      case direction do
        "up" ->
          %{x: 0, y: -step_size}

        "down" ->
          %{x: 0, y: step_size}

        "left" ->
          %{x: -step_size, y: 0}

        "right" ->
          %{x: step_size, y: 0}

        "up-left" ->
          %{x: -step_size, y: -step_size}

        "up-right" ->
          %{x: step_size, y: -step_size}

        "down-left" ->
          %{x: -step_size, y: step_size}

        "down-right" ->
          %{x: step_size, y: step_size}
      end
    else
      case direction do
        "up" ->
          %{x: -step_size, y: -step_size}

        "down" ->
          %{x: step_size, y: step_size}

        "left" ->
          %{x: step_size, y: -step_size}

        "right" ->
          %{x: -step_size, y: step_size}

        "up-left" ->
          %{x: 0, y: -step_size}

        "up-right" ->
          %{x: -step_size, y: 0}

        "down-left" ->
          %{x: step_size, y: 0}

        "down-right" ->
          %{x: 0, y: step_size}
      end
    end
  end
end
