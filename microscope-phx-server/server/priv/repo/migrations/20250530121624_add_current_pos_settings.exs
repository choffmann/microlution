defmodule Server.Repo.Migrations.AddCurrentPosSettings do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :current_x, :integer
      add :current_y, :integer
      add :current_z, :integer

      add :home_sanga_x, :integer
      add :current_sanga_x, :integer

      add :navigate_slider_value, :integer
      add :focus_slider_value, :integer
      add :sanga_slider_value, :integer

      add :minimap_x, :integer
      add :minimap_y, :integer

      add :esp_32_cam_ip, :text
    end
  end
end
