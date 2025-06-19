defmodule ServerWeb.SettingsLive do
  use ServerWeb, :live_view

  alias Server.Settings

  def render(assigns) do
    ~H"""
    <.live_component module={ServerWeb.Components.ConstructionSite} id="construction-site" />
    """
  end

  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end
end
