defmodule ServerWeb.LoggingLive do
  use ServerWeb, :live_view

  alias Server.Api

  def render(assigns) do
    ~H"""
    <.live_component module={ServerWeb.Components.ConstructionSite} id="construction-site" />
    """
  end

  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  def handle_event("logging", _params, socket) do
    logging()
    {:noreply, socket}
  end

  def logging() do
    url = Api.server_logs()

    headers = [
      {"Accept", "application/json"}
    ]

    params = []

    case HTTPoison.get(url, headers) |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body

      _ ->
        IO.inspect("")
        []
    end
  end
end
