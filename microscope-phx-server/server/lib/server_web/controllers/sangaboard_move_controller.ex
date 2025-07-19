defmodule ServerWeb.SangaboardMoveController do
  use ServerWeb, :controller
  alias Server.Navigation
  alias Server.Autofocus

  def move(conn, params) do
    Navigation.move_stage(params["direction"], params["step_size"])
    send_resp(conn, 200, "Moved!")
  end

  def move_focus(conn, params) do
    {type, msg} = Autofocus.adjust_focus(params["step_size"])
    send_resp(conn, 200, "Adjusted focus!")
  end

  def move_slider(conn, params) do
    {type, msg} = Navigation.sanga_move_slider(params["direction"], params["step_size"])
    send_resp(conn, 200, "Moved Slider!")
  end
end
