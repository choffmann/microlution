defmodule ServerWeb.Components.Navigate.NavigateControls do
  use ServerWeb, :live_component

  alias Server.Navigation
  alias Server.Capture
  alias Server.Autofocus

  def render(assigns) do
    ~H"""
    <div class="">
      <div class="col d-flex justify-content-between align-items-center">
        <p class="h4">Navigate Controls</p>
        <button phx-click="handle-navigate-sidebar"><span class="bi-gear fs-3"></span></button>
      </div>
      <div class="col" style="width: 100%">
        <.live_component module={ServerWeb.Components.Navigate.MoveTo} id="move-to" />
      </div>

      <div class="col">
        <div class="row">
          <div class="col-8">
            <.live_component
              module={ServerWeb.Components.Navigate.NavigationArrowPad}
              id="navigation-arrow-pad"
            />
          </div>

          <div class="col-4">
            <.live_component
              module={ServerWeb.Components.Navigate.FocusArrowPad}
              id="focus-arrow-pad"
            />
          </div>
        </div>
      </div>

      <div class="col">
        <.live_component module={ServerWeb.Components.Navigate.NavigationSanga} id="navigate-sanga" />
      </div>

      <div class="col" style="width: 100%">
        <.live_component module={ServerWeb.Components.Navigate.Autofocus} id="autofocus" />
      </div>

      <div class="col">
        <.live_component module={ServerWeb.Components.Navigate.NavigateHome} id="navigate-home" />
      </div>
    </div>
    """
  end

  def mount(socket) do
    {:ok, socket}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end
end
