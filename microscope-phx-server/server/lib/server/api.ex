defmodule Server.Api do
  # def api_ip_address(), do: "192.168.4.1:5000"
  def api_ip_address(), do: "192.168.188.58:5000"
  # def api_ip_address() do
  #   {os, _} = :os.type()

  #   if os == :win32 do
  #     "192.168.188.178:5000"
  #   else
  #     "microscope.local:5000"
  #   end
  # end

  def python() do
    {os, _} = :os.type()

    if os == :win32 do
      "python"
    else
      "python3"
    end
  end

  # def esp_cam_ip_address(), do: "192.168.4.12"
  def esp_cam_ip_address(), do: "192.168.188.55"
  def open_flexure_site(), do: "http://#{api_ip_address()}"
  def esp32_cam_stream, do: "http://#{esp_cam_ip_address()}/stream"
  def esp32_cam_capture, do: "http://#{esp_cam_ip_address()}/capture"
  def camera_stream(), do: "http://#{api_ip_address()}/api/v2/streams/mjpeg"
  def all_images(), do: "http://#{api_ip_address()}/api/v2/captures"
  def capture_image(), do: "http://#{api_ip_address()}/api/v2/actions/camera/capture/"

  def all_images(id, filename),
    do: "http://#{api_ip_address()}/api/v2/captures/#{id}/download/#{filename}"

  def get_image_information_by_id(id), do: "http://#{api_ip_address()}/api/v2/captures/#{id}"
  def delete_image_by_id(id), do: "http://#{api_ip_address()}/api/v2/captures/#{id}"
  def server_logs(), do: "http://#{api_ip_address()}/api/v2/log"

  def move_to_in_image_coords(),
    do:
      "http://#{api_ip_address()}/api/v2/extensions/org.openflexure.camera-stage-mapping/move_in_image_coordinates"

  def move_stage(),
    do: "http://#{api_ip_address()}/api/v2/actions/stage/move/"

  def autofocus(),
    do: "http://#{api_ip_address()}/api/v2/extensions/org.openflexure.autofocus/autofocus"

  def build_zip(),
    do: "http://#{api_ip_address()}/api/v2/extensions/org.openflexure.zipbuilder/build"

  def download_zip(session_id),
    do:
      "http://#{api_ip_address()}/api/v2/extensions/org.openflexure.zipbuilder/get/#{session_id}"
end
