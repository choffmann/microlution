defmodule ServerWeb.Components.CameraStreamMm do
  alias Phoenix.Socket.Broadcast
  use ServerWeb, :live_component
  alias Server.Api
  alias Server.Settings
  alias Server.Navigation

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="d-flex position-absolute">
        <.live_component module={ServerWeb.Components.CameraStream} id="view-camera-stream" />
      </div>
    <%= if @show_mm do %>

      <div
        class="card shadow ml-auto mb-4 mr-4"
        style="height: 20rem; width: 20rem; bottom: 0; right: 0; position: absolute;"
      >


        <div class="d-flex justify-content-around mt-2">
          <button class="" phx-click="handle-esp32-cam-stream" phx-target={@myself}>
          <%= if @stream_mm do %>
              <span class="bi-image fs-3"></span>
          <% else %>
              <span class="bi-camera-video fs-3"></span>
          <% end %>
          </button>

          <button class="" phx-click="esp32-cam-take-capture" phx-target={@myself}>
            <span class="bi-camera fs-3"></span>
          </button>

          <button class="" phx-click="move-slider" phx-target={@myself}>
            <span class="bi-arrow-left fs-3"></span>
          </button>

          <button class="">
            <span class="bi-r-square fs-3" phx-click="reset-minimap" phx-target={@myself}></span>
          </button>

          <button class="" phx-click="handle-show-mm-features" phx-target={@myself}>
          <%= if @show_mm_features do %>
              <span class="bi-eye-slash fs-3"></span>
          <% else %>
              <span class="bi-eye fs-3"></span>
          <% end %>
          </button>

          <div class="d-flex align-items-center">
            <button phx-click="handle-show-mm" phx-target={@myself}>
              <span class="bi-chevron-bar-down fs-5 mr-2"></span>
            </button>
          </div>
        </div>
        <div class="overlay mt-0">
       <%= if @stream_mm do %>
          <img class="d-block minimap-stream-img" src={Api.esp32_cam_stream()} alt="MJPEG stream" />

       <% else %>
          <img class="d-block minimap-stream-img" src={"/images/minimap.jpg"} alt="MJPEG stream" />

       <% end %>
          <canvas
          id="myCanvas"
          class=""
          phx-hook="MiniMap"
          width="320"
          height="220"
          data-boundaryx={@settings.boundary_x}
          data-boundaryy={@settings.boundary_y}
          data-minimapx={@settings.minimap_x}
          data-minimapy={@settings.minimap_y}
          data-showminimapfeatures={Jason.encode!(@settings.show_mm_features)}
          style="top: 34px; left: 0px;"
          >
          </canvas>
        </div>
      </div>
    <% else %>


      <div
        class="card shadow d-flex align-items-center justify-content-center"
        style="height: 4rem; width: 4rem; bottom: 2.5rem; right: 2.5rem; position: absolute;"
      >
          <button phx-click="handle-show-mm" phx-target={@myself}>
            <span class="bi-bullseye fs-1"></span>
          </button>
      </div>
    <% end %>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)

    socket =
      socket
      |> assign(:image_url, "/images/minimap.jpg")
      |> assign(:show_mm, settings.show_mm)
      |> assign(:show_mm_features, settings.show_mm_features)
      |> assign(:stream_mm, settings.stream_mm)
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("handle-show-mm", _params, socket) do
    IO.inspect(socket.assigns.show_mm)
    Settings.update(1, %{"show_mm" => !socket.assigns.show_mm})

    socket =
      socket
      |> assign(:show_mm, !socket.assigns.show_mm)

    {:noreply,
     push_event(
       socket,
       "update-minimap",
       Navigation.refresh_minimap()
     )}
  end

  def handle_event("handle-show-mm-features", _params, socket) do
    IO.inspect(socket.assigns.show_mm_features)
    Settings.update(1, %{"show_mm_features" => !socket.assigns.show_mm_features})

    socket =
      socket
      |> assign(:show_mm_features, !socket.assigns.show_mm_features)

    {:noreply,
     push_event(
       socket,
       "update-minimap",
       Navigation.refresh_minimap()
     )}
  end

  def handle_event("move-slider", _params, socket) do
    Settings.update(1, %{"stream_mm" => !socket.assigns.stream_mm})
    {type, msg} = Navigation.sanga_move_slider("forwards", 42000)

    {:noreply, socket}
  end

  def handle_event("handle-esp32-cam-stream", _params, socket) do
    Settings.update(1, %{"stream_mm" => !socket.assigns.stream_mm})
    socket = socket |> assign(:stream_mm, !socket.assigns.stream_mm)

    {:noreply, socket}
  end

  def handle_event("esp32-cam-take-capture", _params, socket) do
    filename = "./priv/static/images/minimap.jpg"

    case HTTPoison.get(Api.esp32_cam_capture()) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        file = File.write!(filename, body)

        IO.puts("Image saved to #{filename}")

      {:ok, %HTTPoison.Response{status_code: code}} ->
        IO.puts("Failed with status code: #{code}")

      {:error, %HTTPoison.Error{reason: reason}} ->
        IO.puts("Request failed: #{inspect(reason)}")
    end

    socket = socket |> assign(:image_url, "/images/minimap.jpg")

    {:noreply, socket}
  end

  def handle_event("reset-minimap", _params, socket) do
    Settings.update(1, %{"minimap_x" => 0, "minimap_y" => 0})

    {:noreply,
     push_event(
       socket,
       "reset-minimap",
       %{}
     )}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end
end
