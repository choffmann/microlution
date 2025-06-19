defmodule ServerWeb.CaptureLive do
  use ServerWeb, :live_view

  alias Server.Api
  alias Server.Capture

  def render(assigns) do
    ~H"""
    <div class="row h-100 sidebar-2">
      <div class="col-3 ml-3">
        <div class="row h-100 d-flex flex-column gap-2" style="border-right: 1px solid gray;">
          <.simple_form for={@form} phx-change="validate" phx-submit="capture">
            <div class="col-2" style="width: 100%">
              <label for="">Filename</label>
              <.input
                id="filename"
                class="w-100 mb-2"
                field={@form[:filename]}
                type="text"
                placeholder="Leave blank for default"
              />
              <div class="d-flex gap-2 mb-2">
                <.input id="temporary" field={@form[:temporary]} type="checkbox" />
                <label for="temporary">Temporary</label>
              </div>
              <hr />
            </div>

            <div class="col-2" style="width: 100%">
              <div class="d-flex flex-row justify-content-between mb-2">
                <div class="d-flex gap-2">
                  <.input id="full_resolution" field={@form[:full_resolution]} type="checkbox" />
                  <label for="full_resolution">Full resolution</label>
                </div>

                <div class="d-flex gap-2">
                  <.input id="bayer" field={@form[:bayer]} type="checkbox" />
                  <label for="bayer">Store raw data</label>
                </div>
              </div>
              <hr />
            </div>

            <div class="col-2" style="width: 100%">
              <div class="d-flex gap-2">
                <.input id="resize" field={@form[:resize]} type="checkbox" />
                <label for="resize">Resize capture</label>
              </div>

              <div class="d-flex justify-content-around">
                <.input id="width" class="w-25" field={@form[:width]} type="number" />
                <.input id="height" class="w-25" field={@form[:height]} type="number" />
              </div>
            </div>

            <div class="col-2" style="width: 100%">
              <p>Notes</p>
              <textarea class="w-100" name="" id="" placeholder="Capture notes"></textarea>
            </div>

            <div class="col-2" style="width: 100%">
              <p>Annotations</p>
            </div>

            <div class="col-2" style="width: 100%">
              <p>Tags</p>
            </div>
            <hr />
            <div class="col-2" style="width: 100%">
              Stack and Scan
              <div class="row mt-2">
                <div class="col d-flex justify-content-around">
                  <button
                    class="w-100 btn btn-outline-primary"
                    type="submit"
                    phx-disable-with="Capture ..."
                  >
                    CAPTURE
                  </button>
                </div>
              </div>
            </div>
          </.simple_form>
        </div>
      </div>

      <div class="col-8">
        <div class="row d-flex h-100 w-100 flex-wrap flex-column">
          <.live_component module={ServerWeb.Components.CameraStreamMm} id="camera-stream-mm" />
        </div>
      </div>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    form = %{
      "filename" => "",
      "temporary" => "false",
      "full_resolution" => "false",
      "bayer" => "false",
      "resize" => "",
      "height" => "480",
      "width" => "640"
    }

    socket = socket |> assign(:form, to_form(form, as: "capture"))
    {:ok, socket}
  end

  def handle_event("capture", %{"capture" => params}, socket) do
    Capture.capture(params)
    {:noreply, socket}
  end

  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end
end
