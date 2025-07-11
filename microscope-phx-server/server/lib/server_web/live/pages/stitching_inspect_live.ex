defmodule ServerWeb.StitchingInspectLive do
  use ServerWeb, :live_view

  def render(assigns) do
    ~H"""
      <div class="row h-100 sidebar-2">
        <div class="col-3 ml-3">
          <div class="row h-100 d-flex flex-row overflow-auto" style="border-right: 1px solid gray; overflow-y: auto; overflow-x: hidden;">
             <%= for img <- @stitched_images do %>
              <div class="">
                <img class="p-4" src={img} alt="Image" style="height: 25vh;" phx-click="set-image" phx-value-image={img}/>
                <p><%= img |> String.replace_prefix("/images/", "") %></p>
              </div>
              <% end %>
          </div>
        </div>

        <div class="col">
          <div class="row ">
            <img class="p-4 stream-img" src={@selected_image} alt="Image" style="height: 95vh;"/>
          </div>
        </div>
      </div>
    """
  end

  def mount(_params, _session, socket) do
    stitched_images =
      Path.wildcard("priv/static/images/*.{png}")
      |> Enum.map(&String.replace_prefix(&1, "priv/static", ""))
      |> IO.inspect()

    # stitched_images =
    #   Path.wildcard(
    #     "C:/Users/Juli/Repos/microlution/microscope-phx-server/server/priv/static/images/*.{jpg,png}"
    #   )
    #   |> IO.inspect()

    socket =
      socket
      |> assign(:stitched_images, stitched_images)
      |> assign(:selected_image, stitched_images |> List.first())

    {:ok, socket}
  end

  def handle_event("set-image", params, socket) do
    IO.inspect(params["image"])
    socket = socket |> assign(:selected_image, params["image"])
    {:noreply, socket}
  end
end
