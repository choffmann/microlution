defmodule ServerWeb.Esp32CamController do
  use ServerWeb, :controller

  def create(conn, %{"image" => %Plug.Upload{} = upload}) do
    # Save to disk or process here
    save_path = Path.join("priv/static/images", upload.filename)
    File.cp(upload.path, save_path)

    # Respond
    send_resp(conn, 200, "Image received")
  end

  def create(conn, _params) do
    send_resp(conn, 400, "Missing image")
  end
end
