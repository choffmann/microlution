defmodule ServerWeb.NavigateLive do
  use ServerWeb, :live_view

  alias Server.Navigation
  alias Server.Settings
  alias Server.Api

  def render(assigns) do
    ~H"""
    <div class="row h-100 sidebar-2">
      <div class="col-2 ml-3">
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

      <div class="col w-100">
        <div class="row d-flex h-100 w-100 flex-wrap flex-column">
          <div class="col-2 w-100 d-flex gap-3 justify-content-around">
              <div class="">
                <h4>Current Positions</h4>
                <p class="m-0" style={"color: "}>Stage X: {@settings.current_x}</p>
                <p class="m-0">Stage Y: {@settings.current_y}</p>
                <p class="m-0">Stage Z: {@settings.current_z}</p>
                <p class="m-0">Slider X: {@settings.current_sanga_x}</p>
              </div>
              <div class="">
                <h4>Boundaries</h4>
                <p class="m-0">Stage X: +- {@settings.boundary_x}</p>
                <p class="m-0">Stage Y: +- {@settings.boundary_y}</p>
                <p class="m-0">Stage Z: +- {@settings.boundary_z}</p>
                <p class="m-0">Slider X: {@settings.boundary_sanga_end}</p>
              </div>
              <div class="">
                <h4>Navigation</h4>
                <p>Modus: {if @settings.navigation_minimap do "Minimap" else "Kamerastream" end}</p>
                <button class="btn btn-outline-primary" phx-click="set-navigation-type">Modus wechseln</button>
              </div>

          </div>
          <hr/>
          <.live_component module={ServerWeb.Components.CameraStreamMm} id="camera-stream-mm" />
        </div>
      </div>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    settings = Settings.get_settings!(1)

    socket =
      socket
      |> assign(:show_settings, false)
      |> assign(:settings, settings)

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

    if type == :error do
      {:noreply, put_flash(socket, type, msg)}
    else
      {:noreply, socket}
    end
  end

  def handle_info(:update_info, socket) do
    socket =
      socket
      |> assign(:settings, Settings.get_settings!(1))

    {:noreply, socket}
  end

  def handle_event("set-navigation-type", _params, socket) do
    settings = Settings.get_settings!(1)
    Settings.update(1, %{"navigation_minimap" => !settings.navigation_minimap})
    Process.send_after(self(), :update_info, 0)

    {:noreply, socket}
  end

  # def handle_info({:circuits_uart, "ttyAMA0", "stopped\r"}, socket) do
  #   IO.inspect("STOPPED CIRCUITS UART")
  #   {:noreply, socket}
  # end
end
