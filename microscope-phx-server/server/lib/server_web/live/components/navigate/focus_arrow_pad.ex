defmodule ServerWeb.Components.Navigate.FocusArrowPad do
  use ServerWeb, :live_component

  alias Server.Api
  alias Server.Settings
  alias Server.Autofocus

  def render(assigns) do
    ~H"""
    <div class="row gap-2">
      <p class="h5">Fokus</p>

      <div class="row">
        <div class="d-flex flex-column align-items-center gap-2">
          <button
            id="focus-up"
            class="btn btn-outline-primary"
            phx-click="move-z-in-direction"
            phx-hook="HoldFocusButton"
            phx-value-direction="up"
            phx-target={@myself}
          >
            <span class="bi-arrow-up"></span>
          </button>

          <button
            id="focus-down"
            class="btn btn-outline-primary"
            phx-click="move-z-in-direction"
            phx-hook="HoldFocusButton"
            phx-value-direction="custom"
            phx-target={@myself}
          >
            <span class="bi-dot"></span>
          </button>

          <button
            id="focus-down"
            class="btn btn-outline-primary"
            phx-click="move-z-in-direction"
            phx-hook="HoldFocusButton"
            phx-value-direction="down"
            phx-target={@myself}
          >
            <span class="bi-arrow-down"></span>
          </button>

          <div class="d-flex justify-content-center align-items-center">
            <.simple_form
              for={@form_focus_step_size}
              phx-change="set-focus-step-size"
              phx-target={@myself}
            >
              <.input
                id="step-size"
                field={@form_focus_step_size[:focus_step_size]}
                type="range"
                min="1"
                max="10"
                value={@settings.focus_slider_value}
              />
            </.simple_form>

            <p>{@focus_step_size}</p>
          </div>
        </div>
      </div>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)
    form_focus_step_size = %{"focus_step_size" => settings.focus_slider_value}

    socket =
      socket
      |> assign(:form_focus_step_size, to_form(form_focus_step_size))
      |> assign(:focus_step_size, settings.focus_slider_value * settings.focus_step_size)
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("set-focus-step-size", %{"focus_step_size" => focus_step_size}, socket) do
    Settings.update(1, %{"focus_slider_value" => String.to_integer(focus_step_size)})

    socket =
      socket
      |> assign(
        :focus_step_size,
        String.to_integer(focus_step_size) * socket.assigns.settings.focus_step_size
      )

    {:noreply, socket}
  end

  def handle_event("move-z-in-direction", %{"direction" => "up"}, socket) do
    {type, msg} = Autofocus.adjust_focus(-socket.assigns.focus_step_size)
    Process.send_after(self(), {:create_flash, type, msg}, 0)
    {:noreply, socket}
  end

  def handle_event("move-z-in-direction", %{"direction" => "custom"}, socket) do
    {output, _} =
      System.cmd(Api.python(), [
        "./autofocus_py.py",
        "--steps",
        Integer.to_string(-300)
      ])

    {:noreply, socket}
  end

  def handle_event("move-z-in-direction", %{"direction" => "down"}, socket) do
    {type, msg} = Autofocus.adjust_focus(socket.assigns.focus_step_size)
    Process.send_after(self(), {:create_flash, type, msg}, 0)
    {:noreply, socket}
  end
end
