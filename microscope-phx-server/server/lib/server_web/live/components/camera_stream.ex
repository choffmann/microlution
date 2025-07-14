defmodule ServerWeb.Components.CameraStream do
  use ServerWeb, :live_component

  alias Server.Api

  def render(assigns) do
    ~H"""
    <div class="">
      <img class="stream-img" src={Api.camera_stream()} alt="MJPEG stream" style="height: 95vh;" />
    </div>
    """
  end
end
