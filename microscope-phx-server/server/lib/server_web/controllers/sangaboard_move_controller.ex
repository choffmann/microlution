defmodule ServerWeb.SangaboardMoveController do
  use ServerWeb, :controller
  alias Server.Navigation

  def move(conn, params) do
    IO.inspect(params)
    send_resp(conn, 200, "Moved!")
  end

  def move_focus(conn, params) do
    IO.inspect(params)
    send_resp(conn, 200, "Adjusted focus!")
  end
end
