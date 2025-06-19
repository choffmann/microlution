defmodule ServerWeb.Components.Navigate.MoveTo do
  use ServerWeb, :live_component

  alias Server.Api
  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div>
      <p class="h5">Move-to</p>
      <.simple_form for={@form} phx-change="validate" phx-submit="move-to" phx-target={@myself}>
        <div class="d-flex flex-row justify-content-around">
          <div class="w-25 d-flex flex-column">
            <label for="x">X</label>
            <.input id="x" field={@form[:x]} type="number" style="width: 4rem;" value={@x} />
          </div>

          <div class="w-25 d-flex flex-column">
            <label for="y">Y</label>
            <.input id="y" field={@form[:y]} type="number" style="width: 4rem;" value={@y} />
          </div>

          <div class="w-25 d-flex flex-column">
            <label for="z">Z</label>
            <.input id="z" field={@form[:z]} type="number" style="width: 4rem;" value={@z} />
          </div>
        </div>
        <button class="w-100 btn btn-outline-primary mt-3">MOVE</button>
      </.simple_form>
    </div>
    """
  end

  def mount(socket) do
    form = %{"x" => 0, "y" => 0, "z" => 0}
    settings = Settings.get_settings!(1)

    socket =
      socket
      |> assign(:form, to_form(form))
      |> assign(:x, 0)
      |> assign(:y, 0)
      |> assign(:z, 0)
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  def handle_event("move-to", params, socket) do
    a = %{
      # absolute: true,
      x: String.to_integer(params["x"]),
      y: String.to_integer(params["y"]),
      z: String.to_integer(params["z"])
    }

    case HTTPoison.post(
           Api.move_to_in_image_coords(),
           a |> Jason.encode!(),
           [{"Content-Type", "application/json"}]
         )
         |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body |> IO.inspect()

      _ ->
        IO.inspect("")
        []
    end

    {:noreply, socket}
  end
end
