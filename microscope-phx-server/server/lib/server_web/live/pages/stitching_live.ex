defmodule ServerWeb.StitchingLive do
  use ServerWeb, :live_view
  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div class="row h-100 sidebar-2">
      <div class="col-3 ml-3">
        <div class="row h-100 d-flex flex-column" style="border-right: 1px solid gray;">
          <%= if @show_settings do %>
            <.live_component
              module={ServerWeb.Components.Stitching.StitchingSettings}
              id="navigate-settings"
            />
          <% else %>
            <.live_component
              module={ServerWeb.Components.Stitching.StitchingControls}
              id="navigate-controls"
              settings={@settings}
            />
          <% end %>
        </div>
      </div>

      <div class="col-3">
        <div class="row d-flex h-100 w-100 flex-wrap flex-column">
          <.live_component module={ServerWeb.Components.CameraStreamMm} id="camera-stream-mm" />
        </div>
      </div>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    settings = Settings.get_settings!(1)
    socket = socket |> assign(:show_settings, false) |> assign(:settings, settings)
    {:ok, socket}
  end

  def handle_event("handle-stitching-sidebar", _params, socket) do
    socket =
      socket
      |> assign(:show_settings, !socket.assigns.show_settings)

    socket =
      if !socket.assigns.show_settings do
        socket
        |> push_navigate(to: ~p"/stitching")
      else
        socket
      end

    {:noreply, socket}
  end
end
