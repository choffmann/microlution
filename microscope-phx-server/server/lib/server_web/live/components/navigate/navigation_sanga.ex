defmodule ServerWeb.Components.Navigate.NavigationSanga do
  use ServerWeb, :live_component
  alias Server.Settings
  alias Sanga.Board

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="row">
        <div class="d-flex flex-column">
          <div class="col">
            <p class="h5">Move Slider</p>
          </div>

          <div class="col d-flex justify-content-around">
            <button
              class="btn btn-outline-primary"
              phx-click="sanga"
              phx-value-dir="forwards"
              phx-target={@myself}
            >
              <span class="bi-arrow-left fs-4"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="sanga-stop"
              phx-target={@myself}
            >
              <span class="bi-sign-stop fs-4"></span>
            </button>

            <button
              class="btn btn-outline-primary"
              phx-click="sanga"
              phx-value-dir="backwards"
              phx-target={@myself}
            >
              <span class="bi-arrow-right fs-4"></span>
            </button>
          </div>

          <div class="col d-flex justify-content-center align-items-center">
            <.simple_form
              for={@form_sanga_step_size}
              phx-change="set-sanga-step-size"
              phx-target={@myself}
              class="w-75"
            >
              <.input
                id="step-size"
                field={@form_sanga_step_size[:sanga_step_size]}
                type="range"
                min="1"
                max="100"
                value={@settings.sanga_slider_value}
                class=""
              />
            </.simple_form>

            <p class="">{@sanga_step_size}</p>
          </div>

          <p class="h5" style="color: red;">{@sanga_message}</p>
        </div>
      </div>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)
    form_sanga_step_size = %{"sanga_step_size" => settings.sanga_slider_value}

    socket =
      socket
      |> assign(:form_sanga_step_size, to_form(form_sanga_step_size))
      |> assign(:sanga_step_size, settings.sanga_slider_value * settings.sanga_step_size)
      |> assign(:sanga_message, "")
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("set-sanga-step-size", %{"sanga_step_size" => sanga_step_size}, socket) do
    Settings.update(1, %{"sanga_slider_value" => String.to_integer(sanga_step_size)})

    socket =
      socket
      |> assign(
        :sanga_step_size,
        String.to_integer(sanga_step_size) * socket.assigns.settings.sanga_step_size
      )

    {:noreply, socket}
  end

  def handle_event("sanga", %{"dir" => dir}, socket) do
    {os, _} = :os.type()
    sanga_step_size = socket.assigns.sanga_step_size
    settings = Settings.get_settings!(1)

    socket =
      if os == :win32 do
        socket |> assign(:sanga_message, "Sanga ist unter Windows nicht unterstützt.")
      else
        Sanga.Board.start_link()

        if dir == "forwards" do
          Sanga.Board.safe_move_slider(sanga_step_size)

          Settings.update(1, %{
            "current_sanga_x" => settings.current_sanga_x + sanga_step_size
          })

          socket
        else
          Sanga.Board.safe_move_slider(-sanga_step_size)

          Settings.update(1, %{
            "current_sanga_x" => settings.current_sanga_x + -sanga_step_size
          })
        end

        Sanga.Board.stop()
        socket
      end

    {:noreply, socket}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  def handle_event("sanga-stop", _unsigned_params, socket) do
    {os, _} = :os.type()

    Sanga.Board.abort_move()

    {:noreply, socket}
  end
end
