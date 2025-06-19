defmodule Server.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add :home_x, :integer
      add :home_y, :integer
      add :home_z, :integer

      timestamps(type: :utc_datetime)
    end
  end
end
