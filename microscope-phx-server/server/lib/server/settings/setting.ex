defmodule Server.Settings.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field(:home_x, :integer, default: 0)
    field(:home_y, :integer, default: 0)
    field(:home_z, :integer, default: 0)
    field(:current_x, :integer, default: 0)
    field(:current_y, :integer, default: 0)
    field(:current_z, :integer, default: 0)

    field(:home_sanga_x, :integer, default: 0)
    field(:current_sanga_x, :integer, default: 0)

    field(:navigate_slider_value, :integer, default: 0)
    field(:focus_slider_value, :integer, default: 0)
    field(:sanga_slider_value, :integer, default: 0)

    field(:minimap_x, :integer, default: 0)
    field(:minimap_y, :integer, default: 0)

    field(:boundary_x, :integer, default: 0)
    field(:boundary_y, :integer, default: 0)
    field(:boundary_z, :integer, default: 0)
    field(:boundary_sanga_start, :integer, default: 0)
    field(:boundary_sanga_end, :integer, default: 0)

    field(:navigate_step_size, :integer, default: 0)
    field(:focus_step_size, :integer, default: 0)
    field(:sanga_step_size, :integer, default: 0)

    field(:focus_point_x, :integer, default: 0)
    field(:focus_point_y, :integer, default: 0)

    field(:stitching_y_steps, :integer, default: 0)
    field(:stitching_x_steps, :integer, default: 0)
    field(:stitching_sleep_time, :integer, default: 0)
    field(:stitching_step_size, :integer, default: 0)
    field(:stitching_autofocus_type, :string, default: "fast")

    field(:stitching_x_step_boundary, :integer, default: 0)
    field(:stitching_y_step_boundary, :integer, default: 0)
    field(:stitching_sleep_time_boundary, :integer, default: 0)
    field(:stitching_step_size_boundary, :integer, default: 0)

    field(:show_mm, :boolean, default: false)
    field(:show_mm_features, :boolean, default: true)
    field(:stream_mm, :boolean, default: false)

    field(:navigation_minimap, :boolean, default: false)

    field(:esp_32_cam_ip, :string, default: "192.168.188.58")

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [
      :home_x,
      :home_y,
      :home_z,
      :current_x,
      :current_y,
      :current_z,
      :home_sanga_x,
      :current_sanga_x,
      :navigate_slider_value,
      :focus_slider_value,
      :sanga_slider_value,
      :minimap_x,
      :minimap_y,
      :boundary_x,
      :boundary_y,
      :boundary_z,
      :boundary_sanga_start,
      :boundary_sanga_end,
      :navigate_step_size,
      :focus_step_size,
      :sanga_step_size,
      :focus_point_x,
      :focus_point_y,
      :stitching_x_steps,
      :stitching_y_steps,
      :stitching_sleep_time,
      :stitching_step_size,
      :stitching_autofocus_type,
      :stitching_x_step_boundary,
      :stitching_y_step_boundary,
      :stitching_sleep_time_boundary,
      :stitching_step_size_boundary,
      :show_mm,
      :show_mm_features,
      :stream_mm,
      :navigation_minimap,
      :esp_32_cam_ip
    ])
    |> validate_required([
      :home_x,
      :home_y,
      :home_z,
      :current_x,
      :current_y,
      :current_z,
      :home_sanga_x,
      :current_sanga_x,
      :navigate_slider_value,
      :focus_slider_value,
      :sanga_slider_value,
      :minimap_x,
      :minimap_y,
      :boundary_x,
      :boundary_y,
      :boundary_z,
      :boundary_sanga_start,
      :boundary_sanga_end,
      :navigate_step_size,
      :focus_step_size,
      :sanga_step_size,
      :focus_point_x,
      :focus_point_y,
      :stitching_x_steps,
      :stitching_y_steps,
      :stitching_sleep_time,
      :stitching_step_size,
      :stitching_autofocus_type,
      :stitching_x_step_boundary,
      :stitching_y_step_boundary,
      :stitching_sleep_time_boundary,
      :stitching_step_size_boundary,
      :show_mm,
      :show_mm_features,
      :stream_mm,
      :navigation_minimap,
      :esp_32_cam_ip
    ])
  end
end
