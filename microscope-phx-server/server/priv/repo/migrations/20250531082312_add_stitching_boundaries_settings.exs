defmodule Server.Repo.Migrations.AddStitchingBoundariesSettings do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :stitching_x_step_boundary, :integer
      add :stitching_y_step_boundary, :integer
      add :stitching_sleep_time_boundary, :integer
      add :stitching_step_size_boundary, :integer
    end
  end
end
