defmodule ServerWeb.IndexLive do
  use ServerWeb, :live_view

  def render(assigns) do
    ~H"""
    <div class="row d-flex h-100 w-100 flex-wrap flex-column">
      <.live_component module={ServerWeb.Components.CameraStreamMm} id="camera-stream-mm" />
    </div>
    """
  end

  def mount(_params, _session, socket) do
    # Finch.build(:get, "http://192.168.188.61:5000/api/v2/streams/snapshot")
    # |> Finch.request(Server.Finch)
    # |> IO.inspect()

    {:ok, socket}
  end
end
