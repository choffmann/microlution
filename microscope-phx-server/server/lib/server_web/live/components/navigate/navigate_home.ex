defmodule ServerWeb.Components.Navigate.NavigateHome do
  use ServerWeb, :live_component
  alias Server.Navigation
  alias Server.Settings
  alias Server.Autofocus
  alias Server.Capture

  def render(assigns) do
    ~H"""
      <div class="">
        <p class="h4">Home</p>
        <div class="row">
          <div class="col d-flex flex-column gap-3">
            <button class="btn btn-outline-primary" phx-click="set-home" phx-target={@myself}>Set Home Position</button>
            <button class="btn btn-outline-primary" phx-click="move-to-home" phx-target={@myself}>
              Move to Home Position
            </button>
          </div>

          <div class="col d-flex flex-column gap-3">
            <button class="btn btn-outline-primary" phx-click="set-focus-home" phx-target={@myself}>Set Fokus Home Position</button>

            <button class="btn btn-outline-primary" phx-click="move-to-focus-home" phx-target={@myself}>
              Move to Focus Home Position
            </button>
          </div>
        </div>


        <%!-- <button
          class="btn btn-outline-primary"
          type="submit"
          phx-disable-with="Capture ..."
          phx-target={@myself}
        >
          CAPTURE
      </button> --%>
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

  def handle_event("move-to-focus-home", _params, socket) do
    Autofocus.move_to_home()
    socket = socket |> push_navigate(to: ~p"/navigate")
    {:noreply, socket}
  end

  def handle_event("set-home", _params, socket) do
    Navigation.set_home()
    {:noreply, socket |> push_navigate(to: ~p"/navigate")}
  end

  def handle_event("set-focus-home", _params, socket) do
    Settings.update(1, %{"current_z" => 0})
    {:noreply, socket |> push_navigate(to: ~p"/navigate")}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  def handle_event("capture", %{"capture" => params}, socket) do
    Capture.capture(params)
    {:noreply, socket}
  end
end
