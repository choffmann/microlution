defmodule ServerWeb.AboutLive do
  use ServerWeb, :live_view

  def render(assigns) do
    ~H"""
    <.live_component module={ServerWeb.Components.ConstructionSite} id="construction-site" />
    """
  end

  def mount(_params, _session, socket) do
    {:ok, socket}
  end
end
