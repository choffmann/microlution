defmodule ServerWeb.Components.Navigate.NavigateHome do
  use ServerWeb, :live_component
  alias Server.Navigation

  def render(assigns) do
    ~H"""
      <div class="">
              <p class="h5">Home</p>
        <button class="btn btn-outline-primary" phx-click="set-home" phx-target={@myself}>Set Home Position</button>
        <button class="btn btn-outline-primary" phx-click="set-focus-home" phx-target={@myself}>Set Fokus Home Position</button>
        <button class="btn btn-outline-primary" phx-click="move-to-home" phx-target={@myself}>
          Move to Home Position
        </button>
      </div>
    """
  end

  def mount(socket) do
    {:ok, socket}
  end

  def handle_event("move-to-home", _params, socket) do
    Navigation.move_to_home()
    socket = socket |> push_navigate(to: ~p"/navigate")
    {:noreply, socket}
  end

  def handle_event("set-home", _params, socket) do
    Navigation.set_home()
    {:noreply, socket |> push_navigate(to: ~p"/navigate")}
  end

  def handle_event("set-focus-home", _params, socket) do
    Settings.update(1, %{"currenz_z" => 0})
    {:noreply, socket |> push_navigate(to: ~p"/navigate")}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end
end
