defmodule Server.Repo.Migrations.AddSangaBoundaries do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :boundary_sanga_start, :integer
      add :boundary_sanga_end, :integer
    end
  end
end
