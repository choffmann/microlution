defmodule ServerWeb.SangaboardMoveController do
  use ServerWeb, :controller
  alias Server.Navigation
  alias Server.Autofocus

  def move(conn, params) do
    Navigation.move_stage(params["direction"], params["step_size"])
    send_resp(conn, 200, "Moved!")
  end

  def move_focus(conn, params) do
    Autofocus.adjust_focus(params["step_size"])
    send_resp(conn, 200, "Adjusted focus!")
  end
end
