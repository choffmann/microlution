defmodule ServerWeb.GalleryLive do
  use ServerWeb, :live_view
  use HTTPoison.Base

  alias Server.Api

  def render(assigns) do
    ~H"""
    <div class="row h-100 d-flex">
      <div class="col-1 w-100 mt-3" style="height: 5rem;">
        <div class="d-flex justify-content-between">
          <div class="d-flex flex-row gap-5 align-items-center">
            <a href="" phx-click="sort-images" phx-value-sort="asc">
              <span class="bi-arrow-down-short fs-3"></span>
            </a>
            <a href="" phx-click="sort-images" phx-value-sort="desc">
              <span class="bi-arrow-up-short fs-3"></span>
            </a>
            <span>FILTER</span>
          </div>

          <div class="d-flex gap-4 align-items-center">
            <button class="btn btn-outline-danger">CREATE ZIP</button>
          </div>
        </div>

        <div class="border mt-3"></div>
      </div>

      <div class="col d-flex flex-row flex-wrap gap-3 justify-content-center">
        <%= for image <- @images do %>
          <div class="w-25 card">
            <img
              src={image["links"]["download"]["href"]}
              phx-click={show_modal("image-modal-#{image["id"]}")}
              alt=""
              width="100%"
            />
            <div class="d-flex justify-content-between align-items-center">
              <p>{image["name"]}</p>

              <button class="btn" phx-click="delete-image" phx-value-id={image["id"]}>
                <span class="bi-trash-fill fs-3"></span>
              </button>
            </div>

            <p>{image["time"]}</p>

            <button class="btn" phx-click={show_modal("info-image-modal-#{image["id"]}")}>
              More...
            </button>
          </div>

          <.image_modal id={"image-modal-#{image["id"]}"}>
            <img src={image["links"]["download"]["href"]} alt="" />
          </.image_modal>

          <.modal id={"info-image-modal-#{image["id"]}"}>
            HI
          </.modal>
        <% end %>
      </div>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(
        :images,
        get_all()
      )

    {:ok, socket}
  end

  def get_all() do
    url = Api.all_images()

    headers = [
      {"Accept", "application/json"}
    ]

    params = []

    case HTTPoison.get(url, headers) |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body |> Jason.decode!() |> Enum.sort_by(& &1["time"], :desc)

      _ ->
        IO.inspect("")
        []
    end
  end

  def get_image_by_id(id) do
    url = Api.get_image_information_by_id(id)
    headers = []
    params = []

    case HTTPoison.get!(url) |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body |> Jason.decode!() |> IO.inspect()

      _ ->
        IO.inspect("")
        []
    end
  end

  def delete_image_by_id(id) do
    url = Api.delete_image_by_id(id)
    headers = []
    params = []

    case HTTPoison.delete!(url) |> IO.inspect() do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body |> Jason.decode!() |> IO.inspect()

      _ ->
        IO.inspect("")
        []
    end
  end

  # def get_image(id, filename) do
  #   url = "http://192.168.188.61:5000/api/v2/captures/#{id}/download/#{filename}"
  #   headers = []
  #   params = []

  #   "http://192.168.188.61:5000/api/v2/captures/#{id}/download/#{filename}"
  # end

  def handle_event("sort-images", %{"sort" => sort}, socket) do
    socket =
      socket
      |> assign(
        :images,
        Enum.sort_by(
          get_all(),
          & &1["time"],
          if sort == "asc" do
            :asc
          else
            :desc
          end
        )
      )

    {:noreply, socket}
  end

  def handle_event("delete-image", %{"id" => id}, socket) do
    IO.inspect("DELEL")
    delete_image_by_id(id)
    socket = socket |> assign(:images, get_all())
    {:noreply, socket}
  end
end
