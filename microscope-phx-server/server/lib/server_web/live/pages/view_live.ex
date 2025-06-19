defmodule ServerWeb.ViewLive do
  use ServerWeb, :live_view
  alias Server.Api
  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div class="row d-flex h-100 w-100 flex-wrap flex-column">
      <.live_component module={ServerWeb.Components.CameraStreamMm} id="camera-stream-mm" />
    </div>
    """
  end

  def mount(_params, _session, socket) do
    if !Settings.get_settings!(1) do
      Settings.save(%{home_x: 0, home_y: 0, home_z: 0})
    end

    {:ok, socket}
  end
end
