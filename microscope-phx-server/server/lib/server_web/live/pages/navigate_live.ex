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

  def handle_info(:clear_flash, socket) do
    {:noreply, clear_flash(socket)}
  end

  def handle_info({:create_flash, type, msg}, socket) do
    Process.send_after(self(), :clear_flash, 5000)
    {:noreply, put_flash(socket, type, msg)}
  end

  def handle_info({:update_minimap, direction, step_size}, socket) do
    # IO.inspect("Update Minimap: #{direction} - #{step_size}")
    # settings = Settings.get_settings!(1)

    # move_in_direction =
    #   Navigation.get_navigate_direction_minimap(direction, step_size) |> IO.inspect()

    # boundaries = %{boundaryx: settings.boundary_x, boundaryy: settings.boundary_y}

    # Settings.update(1, %{
    #   "minimap_x" => settings.minimap_x + move_in_direction.x,
    #   "minimap_y" => settings.minimap_y + move_in_direction.y
    # })

    # {:noreply,
    #  push_event(
    #    socket,
    #    "update-minimap",
    #    Map.merge(move_in_direction, boundaries)
    #  )}
    {:noreply, socket}
  end

  # def handle_event("update-minimap", _params, socket) do
  #   settings = Settings.get_settings!(1)

  #   move_in_direction =
  #     Navigation.get_navigate_direction_minimap(direction, step_size) |> IO.inspect()

  #   boundaries = %{boundaryx: settings.boundary_x, boundaryy: settings.boundary_y}

  #   Settings.update(1, %{
  #     "minimap_x" => settings.minimap_x + move_in_direction.x,
  #     "minimap_y" => settings.minimap_y + move_in_direction.y
  #   })

  #   {:noreply,
  #    push_event(
  #      socket,
  #      "update-minimap",
  #      Map.merge(move_in_direction, boundaries)
  #    )}
  # end

  def handle_info({:circuits_uart, "ttyAMA0", "stopped\r"}, socket) do
    IO.inspect("STOPPED CIRCUITS UART")
    {:noreply, socket}
  end
end
