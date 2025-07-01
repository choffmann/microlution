defmodule ServerWeb.Components.Navigate.NavigationArrowPad do
  use ServerWeb, :live_component

  alias Server.Api
  alias Server.Navigation
  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div>
      <div class="row gap-2">
        <p class="h5">Navigation</p>

        <div class="row ">
          <div class="d-flex justify-content-around">
            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="up-left"
              phx-target={@myself}
            >
              <span class="bi-arrow-up-left"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="up"
              phx-target={@myself}
            >
              <span class="bi-arrow-up"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="up-right"
              phx-target={@myself}
            >
              <span class="bi-arrow-up-right"></span>
            </button>
          </div>
        </div>

        <div class="row">
          <div class="d-flex justify-content-around">
            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="left"
              phx-target={@myself}
            >
              <span class="bi-arrow-left"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="left"
              phx-target={@myself}
              disabled
            >
              <span class="bi-dot"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="right"
              phx-target={@myself}
            >
              <span class="bi-arrow-right"></span>
            </button>
          </div>
        </div>

        <div class="row gap-2">
          <div class="d-flex justify-content-around">
            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="down-left"
              phx-target={@myself}
            >
              <span class="bi-arrow-down-left"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="down"
              phx-target={@myself}
            >
              <span class="bi-arrow-down"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="move-in-direction"
              phx-value-direction="down-right"
              phx-target={@myself}
            >
              <span class="bi-arrow-down-right"></span>
            </button>
          </div>
        </div>

        <div class="row">
          <div class="d-flex justify-content-center align-items-center">
            <.simple_form for={@form_step_size} phx-change="set-step-size" phx-target={@myself}>
              <.input
                id="step-size"
                field={@form_step_size[:step_size]}
                type="range"
                min="1"
                max="50"
                class="w-75"
                value={@settings.navigate_slider_value}
              />
            </.simple_form>

            <p>{@step_size}</p>
          </div>
        </div>
        <div class="row">
          <p>Boundary X: +- {@settings.boundary_x}</p>
          <p>Boundary Y: +- {@settings.boundary_y}</p>
          <%!-- <p>Boundary Z: +- {@settings.boundary_z}</p> --%>
          <p>Current X: {@settings.current_x}</p>
          <p>Current Y: {@settings.current_y}</p>
          <%!-- <p>Current Z: {@settings.current_z}</p> --%>
        </div>
      </div>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)
    form_step_size = %{"step_size" => settings.navigate_slider_value}

    socket =
      socket
      |> assign(:form_step_size, to_form(form_step_size))
      |> assign(:step_size, settings.navigate_slider_value * settings.navigate_step_size)
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("set-step-size", %{"step_size" => step_size}, socket) do
    Settings.update(1, %{"navigate_slider_value" => String.to_integer(step_size)})

    socket =
      socket
      |> assign(
        :step_size,
        String.to_integer(step_size) * socket.assigns.settings.navigate_step_size
      )

    {:noreply, socket}
  end

  def handle_event("move-in-direction", %{"direction" => direction}, socket) do
    settings = Settings.get_settings!(1)
    step_size = socket.assigns.step_size

    Navigation.move_stage(direction, step_size)

    socket = socket |> assign(:settings, Settings.get_settings!(1))
    {:noreply, socket}
  end
end
