defmodule ServerWeb.NavigateLive do
  use ServerWeb, :live_view

  alias Server.Navigation
  alias Server.Settings
  alias Server.Api

  def render(assigns) do
    ~H"""
    <div class="row h-100 sidebar-2">
      <div class="col-3 ml-3">
        <div class="row h-100 d-flex flex-column" style="border-right: 1px solid gray;">
          <%= if @show_settings do %>
            <.live_component
              module={ServerWeb.Components.Navigate.NavigateSettings}
              id="navigate-settings"
            />
          <% else %>
            <.live_component
              module={ServerWeb.Components.Navigate.NavigateControls}
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

  def handle_event("handle-navigate-sidebar", _params, socket) do
    socket = socket |> assign(:show_settings, !socket.assigns.show_settings)
    {:noreply, socket}
  end

  def handle_info({:update_minimap, direction, step_size}, socket) do
    IO.inspect("Update Minimap: #{direction} - #{step_size}")

    move_in_direction =
      Navigation.get_navigate_direction_minimap(direction, step_size) |> IO.inspect()

    {:noreply,
     push_event(
       socket,
       "update-minimap",
       move_in_direction
     )}
  end

  def handle_event("update-minimap", _params, socket) do
    {:noreply,
     push_event(
       socket,
       "update-minimap",
       %{x: Jason.encode!(50), y: Jason.encode!(50)}
     )}
  end
end
