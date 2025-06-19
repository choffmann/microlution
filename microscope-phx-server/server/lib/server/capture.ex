defmodule Server.Capture do
  alias Server.Api

  def capture(params) do
    case HTTPoison.post(
           Api.capture_image(),
           Jason.encode(%{
             annotations: %{
               Client: "SwaggerUI"
             },
             bayer: params["bayer"],
             filename: params["filename"],
             resize: %{
               height:
                 if params["full_resolution"] do
                   1944
                 else
                   params["height"]
                 end,
               width:
                 if params["full_resolution"] do
                   2592
                 else
                   params["width"]
                 end
             },
             tags: [
               "docs"
             ],
             temporary: params["temporary"],
             use_video_port: false
           })
           |> elem(1),
           [{"Content-Type", "application/json"}]
         ) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        IO.inspect(body)

      {:ok, %HTTPoison.Response{status_code: 201, body: body}} ->
        Jason.decode!(body)["id"]

      _ ->
        IO.inspect("")
        []
    end
  end
end
