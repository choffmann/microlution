defmodule Server.Repo.Migrations.AddStitchingSettings do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :stitching_y_steps, :integer
      add :stitching_x_steps, :integer
      add :stitching_sleep_time, :integer
      add :stitching_step_size, :integer
      add :stitching_autofocus_type, :text
    end
  end
end
