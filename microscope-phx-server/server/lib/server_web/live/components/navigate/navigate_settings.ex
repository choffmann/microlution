defmodule ServerWeb.Components.Navigate.NavigateSettings do
  use ServerWeb, :live_component
  alias Server.Settings

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="col d-flex justify-content-between align-items-center">
        <p class="h4">Navigate Settings</p>
        <button phx-click="handle-navigate-sidebar"><span class="bi-gear fs-3"></span></button>
      </div>
      <.simple_form for={@navigate_settings_form} phx-change="validate" phx-target={@myself}>
        <div class="col-2 w-100">
          <p class="h5">Boundaries</p>

          <.input
            id="boundary_x"
            label="X"
            field={@navigate_settings_form[:boundary_x]}
            type="number"
            value={@settings.boundary_x}
          />
          <.input
            id="boundary_y"
            label="Y"
            field={@navigate_settings_form[:boundary_y]}
            type="number"
            value={@settings.boundary_y}
          />
          <.input
            id="boundary_z"
            label="Z"
            field={@navigate_settings_form[:boundary_z]}
            type="number"
            value={@settings.boundary_z}
          />
        </div>

        <div class="col-2 w-100">
          <p class="h5">Step Sizes</p>

          <.input
            id="navigate-step-size"
            label="Navigate Step Size"
            field={@navigate_settings_form[:navigate_step_size]}
            type="number"
            value={@settings.navigate_step_size}
          />
          <.input
            id="focus-step-size"
            label="Focus Step Size"
            field={@navigate_settings_form[:focus_step_size]}
            type="number"
            value={@settings.focus_step_size}
          />
          <.input
            id="sanga-step-size"
            label="Sanga Step Size"
            field={@navigate_settings_form[:sanga_step_size]}
            type="number"
            value={@settings.sanga_step_size}
          />
        </div>
      </.simple_form>
    </div>
    """
  end

  def mount(socket) do
    settings = Settings.get_settings!(1)

    navigate_settings_form = %{
      "boundary_x" => 0,
      "boundary_y" => 0,
      "boundary_z" => 0,
      "navigate_step_size" => 0,
      "focus_step_size" => 0,
      "sange_step_size" => 0
    }

    socket =
      socket
      |> assign(:navigate_settings_form, to_form(navigate_settings_form))
      |> assign(:settings, settings)

    {:ok, socket}
  end

  def handle_event("validate", params, socket) do
    Settings.update(1, params)

    Phoenix.PubSub.broadcast(
      Server.PubSub,
      "update-minimap",
      {:update_minimap, "right", 0}
    )

    {:noreply, socket}
  end
end
