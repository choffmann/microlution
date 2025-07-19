defmodule ServerWeb.Components.Stitching.StitchingControls do
  use ServerWeb, :live_component

  alias Server.Navigation
  alias Server.Capture
  alias Server.Autofocus
  alias Server.Settings
  alias Server.Api

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="col d-flex justify-content-between align-items-center">
        <p class="h4">Stitching Controls</p>
         <button phx-click="handle-stitching-sidebar"><span class="bi-gear fs-3"></span></button>
      </div>

      <.simple_form for={@stitching_controls_form} phx-change="validate" phx-target={@myself}>
        <div class="col-2 w-100">
          <p class="h5"></p>

          <div class="d-flex justify-content-between">
            <label for="x-steps">X-Images</label>
            <div class="d-flex align-items-center">
              <.input
                id="x-steps"
                field={@stitching_controls_form[:x_steps]}
                type="range"
                min="1"
                max={@settings.stitching_x_step_boundary}
                value={@x_steps}
              />
              <p>{@x_steps}</p>
            </div>
          </div>

          <div class="d-flex justify-content-between">
            <label for="x-steps">Y-Images</label>
            <div class="d-flex align-items-center">
              <.input
                id="y-steps"
                field={@stitching_controls_form[:y_steps]}
                type="range"
                min="1"
                max={@settings.stitching_y_step_boundary}
                value={@y_steps}
              />
              <p>{@y_steps}</p>
            </div>
          </div>

          <div class="d-flex justify-content-between">
            <label for="x-steps">Sleep between images</label>
            <div class="d-flex align-items-center">
              <.input
                id="sleep-time"
                field={@stitching_controls_form[:sleep_time]}
                type="range"
                min="1"
                max={@settings.stitching_sleep_time_boundary}
                value={@sleep_time}
              />
              <p>{@sleep_time}</p>
            </div>
          </div>

          <div class="d-flex justify-content-between">
            <label for="x-steps">Step-Size between images</label>
            <div class="d-flex align-items-center">
              <.input
                id="step-size"
                field={@stitching_controls_form[:step_size]}
                type="range"
                min="0"
                max={@settings.stitching_step_size_boundary}
                value={@step_size}
              />
              <p>{@step_size}</p>
            </div>
          </div>

          <div class="d-flex justify-content-between">
            <label for="x-steps">Autofocus Type</label>
            <div class="d-flex align-items-center">
              <.input
                field={@stitching_controls_form[:autofocus_type]}
                type="select"
                options={Enum.map(@autofocus_types, &{&1.type, &1.id})}
                value={Enum.at(@autofocus_types, @selected_autofocus_type).type}
                phx-change="validate"
                style="width: 10rem;"
              />
            </div>
          </div>

        </div>

        <div class="col w-100">
          <p class="h5">Generated Tiles Preview</p>
          <canvas class="w-100" id="stitching-boxes-preview" phx-hook="StitchingBoxesPreview" height="200"></canvas>
        </div>
      </.simple_form>

      <div class="col">
        <.live_component module={ServerWeb.Components.Navigate.NavigateHome} id="navigate-home" />
      </div>

      <div class="col">
        <p class="h5">Stitching</p>

        <button class="btn btn-outline-primary" phx-click="start-stitching" phx-target={@myself}>
          Start
        </button>
        <button class="btn btn-outline-primary" phx-click="stop-stitching" phx-target={@myself}>
          Stop
        </button>
      </div>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)

    stitching_controls_form = %{
      "x_steps" => settings.stitching_x_steps,
      "y_steps" => settings.stitching_y_steps,
      "step_size" => settings.stitching_step_size,
      "sleep_time" => settings.stitching_sleep_time,
      "autofocus_type" => settings.stitching_autofocus_type
    }

    autofocus_types = [
      %{type: "fast", id: :fast},
      %{type: "medium", id: :medium},
      %{type: "fine", id: :fine}
    ]

    selected_autofocus_type =
      case settings.stitching_autofocus_type do
        "fast" -> 0
        "medium" -> 1
        "fine" -> 2
      end

    socket =
      socket
      |> assign(:stitching_controls_form, to_form(stitching_controls_form))
      |> assign(:stitching_on, true)
      |> assign(:autofocus_type, settings.stitching_autofocus_type)
      |> assign(:autofocus_types, autofocus_types)
      |> assign(:sleep_time, settings.stitching_sleep_time)
      |> assign(:step_size, settings.stitching_step_size)
      |> assign(:x_steps, settings.stitching_x_steps)
      |> assign(:y_steps, settings.stitching_y_steps)
      |> assign(:settings, settings)
      |> assign(:selected_autofocus_type, selected_autofocus_type)

    {:ok,
     push_event(
       socket,
       "update-stitching-preview-boxes",
       %{
         x: Jason.encode!(settings.stitching_x_steps),
         y: Jason.encode!(settings.stitching_y_steps)
       }
     )}
  end

  def handle_event("stop-stitching", _params, socket) do
    socket = socket |> assign(:stitching_on, false)
    {:noreply, socket}
  end

  def handle_event("validate", params, socket) do
    IO.inspect(params["autofocus_type"])

    Settings.update(1, %{
      "stitching_x_steps" => params["x_steps"],
      "stitching_y_steps" => params["y_steps"],
      "stitching_sleep_time" => params["sleep_time"],
      "stitching_step_size" => params["step_size"],
      "stitching_autofocus_type" => params["autofocus_type"]
    })

    socket =
      socket
      |> assign(:stitching_on, true)
      |> assign(:autofocus_type, params["autofocus_type"])
      |> assign(:sleep_time, String.to_integer(params["sleep_time"]))
      |> assign(:step_size, String.to_integer(params["step_size"]))
      |> assign(:x_steps, String.to_integer(params["x_steps"]))
      |> assign(:y_steps, String.to_integer(params["y_steps"]))

    {:noreply,
     push_event(
       socket,
       "update-stitching-preview-boxes",
       %{
         x: Jason.encode!(String.to_integer(params["x_steps"])),
         y: Jason.encode!(String.to_integer(params["y_steps"]))
       }
     )}
  end

  def stitching_run_x(
        start_num,
        end_num,
        stitching_on,
        step_size,
        sleep_time,
        autofocus_type,
        direction,
        y
      ) do
    Enum.map(start_num..end_num, fn x ->
      if stitching_on do
        Autofocus.autofocus(autofocus_type)
        :timer.sleep(sleep_time * 1000)

        datetime_string = DateTime.utc_now() |> DateTime.to_string()

        filename =
          "tile_#{if direction == "left" do
            end_num - x
          else
            x
          end}_#{y}"

        image_id =
          Capture.capture(%{
            "filename" => filename,
            "temporary" => "false",
            "full_resolution" => "false",
            "bayer" => "false",
            "resize" => "true",
            "height" => "480",
            "width" => "640"
          })

        if x < end_num do
          Navigation.move_stage(direction, step_size)
          :timer.sleep(3000)
        else
          Navigation.move_stage("down", step_size)
          :timer.sleep(3000)
        end

        filename
      end
    end)
  end

  def stitching_run_y(socket) do
    Enum.map(0..(socket.assigns.y_steps - 1), fn x ->
      direction =
        if rem(x, 2) == 0 do
          "right"
        else
          "left"
        end

      stitching_run_x(
        start_num = 0,
        end_num =
          socket.assigns.x_steps - 1,
        stitching_on = socket.assigns.stitching_on,
        step_size = socket.assigns.step_size,
        sleep_time = socket.assigns.sleep_time,
        autofocus_type = socket.assigns.autofocus_type,
        direction = direction,
        y = x
      )
    end)
  end

  def handle_event("start-stitching", _params, socket) do
    image_filenames =
      stitching_run_y(socket)
      |> List.flatten()
      |> IO.inspect()

    run_id = DateTime.utc_now() |> DateTime.to_unix()
    base_path = Path.join(System.user_home!(), "stitching/#{run_id}")
    zip_path = Path.join(base_path, "tiles.zip")

    File.mkdir_p!(base_path)

    if not File.dir?(
         "/home/pi/niklas/microlution/microscope-phx-server/server/priv/static/images/stitched_images/"
       ) do
      System.shell(
        "mkdir /home/pi/niklas/microlution/microscope-phx-server/server/priv/static/images/stitched_images/"
      )
    end

    Enum.map(image_filenames, fn filename ->
      System.shell(
        "cp /var/openflexure/data/micrographs/#{filename}.jpeg /home/pi/niklas/microlution/microscope-phx-server/server/priv/static/images/stitched_images/"
      )
    end)

    {output, code} =
      System.cmd(
        "python3",
        [
          "/home/pi/niklas/microlution/stitching/stitching_orb.py",
          "/home/pi/niklas/microlution/microscope-phx-server/server/priv/static/images/stitched_images/"
        ],
        stderr_to_stdout: true
      )

    IO.puts("Stitching-Ausgabe:")
    IO.puts(output)

    if code == 0 do
      IO.puts("Stitching erfolgreich!")
    else
      IO.puts("Fehler beim Stitching (Exit-Code #{code})")
    end

    Enum.map(image_filenames, fn filename ->
      System.shell(
        "rm -r /home/pi/niklas/microlution/microscope-phx-server/server/priv/static/images/stitched_images/#{filename}.jpeg"
      )
    end)

    # case HTTPoison.post(Api.build_zip(), Jason.encode!(image_ids), [
    #        {"Content-Type", "application/json"}
    #      ]) do
    #   {:ok, %HTTPoison.Response{status_code: 201, body: body}} ->
    #     session_id = Jason.decode!(body)["output"]["id"]
    #     download_url = Api.download_zip(session_id)

    #     case HTTPoison.get(download_url) do
    #       {:ok, %HTTPoison.Response{status_code: 200, body: zip_binary}} ->
    #         File.write!(zip_path, zip_binary)
    #         IO.puts("ZIP gespeichert: #{zip_path}")

    #         {output, code} =
    #           System.cmd(
    #             "python3",
    #             [
    #               "/home/pi/niklas/microlution/stitching/stitching_like_stitch2d.py",
    #               "/home/pi/stitch/tiles.zip"
    #             ],
    #             stderr_to_stdout: true
    #           )

    #         IO.puts("Stitching-Ausgabe:")
    #         IO.puts(output)

    #         if code == 0 do
    #           IO.puts("Stitching erfolgreich!")
    #         else
    #           IO.puts("Fehler beim Stitching (Exit-Code #{code})")
    #         end

    #       err ->
    #         IO.inspect(err, label: "Fehler beim Herunterladen der ZIP-Datei")
    #     end

    #   err ->
    #     IO.inspect(err, label: "Fehler beim Erzeugen der ZIP-Datei")
    # end

    # case HTTPoison.post(
    #        Api.build_zip(),
    #        image_ids |> Jason.encode!(),
    #        [{"Content-Type", "application/json"}]
    #      )
    #      |> IO.inspect() do
    #   {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
    #     body |> IO.inspect()
    #
    #   {:ok, %HTTPoison.Response{status_code: 201, body: body}} ->
    #     id = Jason.decode!(body)["output"]["id"]
    #
    #   case HTTPoison.get(Api.download_zip(id))
    #        |> IO.inspect() do
    #     {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
    #       #File.write("me.zip", body) |> IO.inspect()
    #
    #     {:ok, %HTTPoison.Response{status_code: 201, body: body}} ->
    #       #File.write("me", body) |> IO.inspect()
    #
    #     _ ->
    #       []
    #   end
    #
    #   _ ->
    #     []
    # end

    # Enum.each(0..5, fn x ->
    #   Autofocus.autofocus("fast")
    #   :timer.sleep(10000)

    #   datetime_string = DateTime.utc_now() |> DateTime.to_string()

    #   Capture.capture(%{
    #     "filename" => "Stitching no. #{x} - #{datetime_string}",
    #     "temporary" => "false",
    #     "full_resolution" => "false",
    #     "bayer" => "false",
    #     "resize" => "",
    #     "height" => "480",
    #     "width" => "640"
    #   })

    #   if x < 5 do
    #     Navigation.move_in_direction("right", step_size)
    #   end
    # end)

    # Navigation.move_in_direction("down", step_size)

    # Enum.each(6..11, fn x ->
    #   Autofocus.autofocus("fast")
    #   :timer.sleep(10000)
    #   datetime_string = DateTime.utc_now() |> DateTime.to_string()

    #   Capture.capture(%{
    #     "filename" => "Stitching no. #{x} - #{datetime_string}",
    #     "temporary" => "false",
    #     "full_resolution" => "false",
    #     "bayer" => "false",
    #     "resize" => "",
    #     "height" => "480",
    #     "width" => "640"
    #   })

    #   if x < 11 do
    #     Navigation.move_in_direction("left", step_size)
    #   end
    # end)

    # Navigation.move_in_direction("down", step_size)

    # Enum.each(12..17, fn x ->
    #   Autofocus.autofocus("fast")
    #   :timer.sleep(10000)
    #   datetime_string = DateTime.utc_now() |> DateTime.to_string()

    #   Capture.capture(%{
    #     "filename" => "Stitching no. #{x} - #{datetime_string}",
    #     "temporary" => "false",
    #     "full_resolution" => "false",
    #     "bayer" => "false",
    #     "resize" => "",
    #     "height" => "480",
    #     "width" => "640"
    #   })

    #   if x < 17 do
    #     Navigation.move_in_direction("right", step_size)
    #   end
    # end)

    {:noreply, socket}
  end
end
