defmodule Server.Repo.Migrations.AddFurtherSettings do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :boundary_x, :integer
      add :boundary_y, :integer
      add :boundary_z, :integer

      add :navigate_step_size, :integer
      add :focus_step_size, :integer
      add :sanga_step_size, :integer

      add :focus_point_x, :integer
      add :focus_point_y, :integer
    end
  end
end
