defmodule ServerWeb.Components.Stitching.StitchingSettings do
  use ServerWeb, :live_component

  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="col d-flex justify-content-between align-items-center">
        <p class="h4">Stitching Settings</p>
         <button phx-click="handle-stitching-sidebar"><span class="bi-gear fs-3"></span></button>
      </div>

      <.simple_form for={@stitching_controls_boundary_form} phx-change="validate" phx-target={@myself}>
            <div class="col-2 w-100">
              <p class="h5">Stitching Boundaries</p>

              <.input
                id="stitching-x-step-boundary"
                label="Max. X-Images"
                field={@stitching_controls_boundary_form[:stitching_x_step_boundary]}
                type="number"
                value={@settings.stitching_x_step_boundary}
              />
              <.input
                id="stitching-y-step-boundary"
                label="Max. Y-Images"
                field={@stitching_controls_boundary_form[:stitching_y_step_boundary]}
                type="number"
                value={@settings.stitching_y_step_boundary}
              />
              <.input
                id="stitching-sleep-time-boundary"
                label="Max. Sleep-Time"
                field={@stitching_controls_boundary_form[:stitching_sleep_time_boundary]}
                type="number"
                value={@settings.stitching_sleep_time_boundary}
              />
                            <.input
                id="stitching-step-size-boundary"
                label="Max. Step-Size"
                field={@stitching_controls_boundary_form[:stitching_step_size_boundary]}
                type="number"
                value={@settings.stitching_step_size_boundary}
              />
            </div>

          </.simple_form>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)

    stitching_controls_boundary_form = %{
      "stitching_x_step_boundary" => settings.stitching_x_step_boundary,
      "stitching_y_step_boundary" => settings.stitching_y_step_boundary,
      "stitching_sleep_time_boundary" => settings.stitching_sleep_time_boundary,
      "stitching_step_size_boundary" => settings.stitching_step_size_boundary
    }

    socket =
      socket
      |> assign(:settings, settings)
      |> assign(:stitching_controls_boundary_form, to_form(stitching_controls_boundary_form))

    {:ok, socket}
  end

  def handle_event("validate", params, socket) do
    Settings.update(1, params)
    settings = Settings.get_settings!(1)
    IO.inspect(settings.stitching_x_steps)
    IO.inspect(params["stitching_x_step_boundary"])

    if settings.stitching_x_steps > String.to_integer(params["stitching_x_step_boundary"]) do
      Settings.update(1, %{"stitching_x_steps" => params["stitching_x_step_boundary"]})
    end

    if settings.stitching_y_steps > String.to_integer(params["stitching_y_step_boundary"]) do
      Settings.update(1, %{"stitching_y_steps" => params["stitching_y_step_boundary"]})
    end

    if settings.stitching_sleep_time > String.to_integer(params["stitching_sleep_time_boundary"]) do
      Settings.update(1, %{"stitching_sleep_time" => params["stitching_sleep_time_boundary"]})
    end

    if settings.stitching_step_size > String.to_integer(params["stitching_step_size_boundary"]) do
      Settings.update(1, %{"stitching_step_size" => params["stitching_step_size_boundary"]})
    end

    {:noreply, socket}
  end
end
