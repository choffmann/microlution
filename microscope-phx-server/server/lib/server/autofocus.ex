defmodule Server.Autofocus do
  alias Server.Api
  alias Server.Settings

  def autofocus(type) do
    case HTTPoison.post(
           Api.autofocus(),
           case type do
             "fast" ->
               autofocus_type_fast_body()

             "medium" ->
               autofocus_type_medium_body()

             "fine" ->
               autofocus_type_fine_body()

             "custom" ->
               custom_autofocus()
           end
           |> Jason.encode!(),
           [{"Content-Type", "application/json"}]
         )
         |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body |> IO.inspect()

      _ ->
        IO.inspect("")
        []
    end
  end

  def autofocus_type_fast_body() do
    %{
      dz: [
        -300,
        -200,
        -100,
        0,
        100,
        200,
        300
      ]
    }
  end

  def autofocus_type_medium_body() do
    %{
      dz: [2000]
    }
  end

  def autofocus_type_fine_body() do
    %{
      backlash: 25,
      dz: [2000],
      initial_move_up: true,
      target_z: -100
    }
  end

  def custom_autofocus() do
    video_path = "./autofocus1.mp4"
    adjust_focus(4000)
    {output, 0} = System.cmd("python", ["./autofocus.py"])
    IO.inspect(output)
    #     System.shell(
    #   "cd .. && cd .. && autofocus.py #{video_path}"
    # )
    # output |> String.trim() |> String.to_integer()
  end

  def adjust_focus(focus_step_size) do
    settings = Settings.get_settings!(1)

    cond do
      focus_step_size > 0 and settings.current_z + focus_step_size < settings.boundary_z ->
        Settings.update(1, %{
          "current_z" => settings.current_z + focus_step_size
        })

        Sanga.Board.safe_move_stage_z(focus_step_size)

      focus_step_size < 0 and settings.current_z + focus_step_size > -settings.boundary_z ->
        Settings.update(1, %{
          "current_z" => settings.current_z + focus_step_size
        })

        Sanga.Board.safe_move_stage_z(focus_step_size)
    end
  end
end
