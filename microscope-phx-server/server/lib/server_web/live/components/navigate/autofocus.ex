defmodule ServerWeb.Components.Navigate.Autofocus do
  use ServerWeb, :live_component

  alias Server.Api
  alias Server.Settings
  alias Server.Autofocus

  def render(assigns) do
    ~H"""
    <div>
      <p class="h5">Autofocus</p>

      <div class="row mt-2">
        <div class="col d-flex justify-content-around">
          <button
            class="btn btn-outline-primary"
            phx-click="autofocus"
            phx-value-type="fast"
            phx-target={@myself}
          >
            FAST
          </button>

          <%!-- <button
            class="btn btn-outline-primary"
            phx-click="autofocus"
            phx-value-type="medium"
            phx-target={@myself}
          >
            MEDIUM
          </button>

          <button
            class="btn btn-outline-primary"
            phx-click="autofocus"
            phx-value-type="fine"
            phx-target={@myself}
          >
            FINE
          </button>
                    <button
            class="btn btn-outline-primary"
            phx-click="autofocus"
            phx-value-type="custom"
            phx-target={@myself}
          >
            CUSTOM
          </button> --%>
        </div>
      </div>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)
    socket = socket |> assign(:settings, settings)
    {:ok, socket}
  end

  def handle_event("autofocus", %{"type" => type}, socket) do
    Autofocus.autofocus(type)

    {:noreply, socket}
  end
end
