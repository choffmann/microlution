defmodule ServerWeb.StitchingInspectLive do
  use ServerWeb, :live_view

  def render(assigns) do
    ~H"""
    <div class="row h-100 sidebar-2">
      <div class="col-3 ml-3">
        <div class="row h-100 d-flex flex-row overflow-auto gap-3 p-3" style="max-height: 100vh; border-right: 1px solid gray; overflow-y: auto; overflow-x: hidden;">
           <%= for {img, id} <- @stitched_images do %>
           <div class="card d-flex flex-column justify-content-center align-items-center" phx-click="set-image" phx-value-image={img}  style={"background-color: #{if img |> String.replace_prefix("/images/stitched_images/", "") == @selected_image|> String.replace_prefix("/images/stitched_images/", "") do "lightgrey" else "white" end};"}>
              <img class="p-4" src={img} alt="Image" />
              <p><%= img |> String.replace_prefix("/images/stitched_images/", "") %></p>
              <button phx-click={show_modal("delete-stitched-image-confirm-#{id}")}>
                <span class="bi-trash fs-3"></span>
              </button>
              <.modal id={"delete-stitched-image-confirm-#{id}"}>
                <button phx-click="delete-stitched-image" phx-value-id={id}>
                  <span class="bi-trash fs-3"></span>
                </button>
              </.modal>
           </div>
            <% end %>
        </div>
      </div>

      <div class="col">
          <div class="h-100 d-flex justify-content-center align-items-center">
           <%= if @selected_image != "" do %>
            <div id="openseadragon" phx-hook="StitchingInspector" data-image={@selected_image} style="width: 100%; height: 100vh;"></div>

           <% else %>
            <p class="h3">Select an Image</p>
           <% end %>
          </div>
      </div>



    </div>
    """
  end

  def mount(_params, _session, socket) do
    magnifier_form = %{"zoom" => 1, "size" => 100}

    stitched_images =
      Path.wildcard("priv/static/images/stitched_images/stitched*.{png}")
      |> Enum.map(&String.replace_prefix(&1, "priv/static", ""))
      |> Enum.with_index()
      |> IO.inspect()

    socket =
      socket
      |> assign(:stitched_images, stitched_images)
      |> assign(:selected_image, "")
      |> assign(:zoom, 1)
      |> assign(:size, 100)
      |> assign(:magnifier_form, to_form(magnifier_form))

    {:ok, socket}
  end

  def handle_event("set-image", params, socket) do
    selected_image =
      if params["image"] == socket.assigns.selected_image do
        ""
      else
        params["image"]
      end

    socket = socket |> assign(:selected_image, selected_image)

    {:noreply,
     push_event(
       socket,
       "update-stitching-inspector",
       %{"image" => params["image"]}
     )}
  end

  def handle_event("delete-stitched-image", %{"id" => id, "value" => ""}, socket) do
    image_to_delete =
      Path.wildcard("priv/static/images/stitched_images/stitched*.{png}")
      |> Enum.map(&String.replace_prefix(&1, "priv/static", ""))
      |> Enum.with_index()
      |> Enum.find(fn {img, id} -> id == id end)
      |> elem(0)
      |> String.replace_prefix("/images/stitched_images/", "")

    System.shell(
      "rm -r #{Path.wildcard("priv/static/images/stitched_images/")}/#{image_to_delete}"
    )

    {:noreply, socket}
  end
end
