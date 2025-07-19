defmodule Server.Repo.Migrations.AddNavigationMinimap do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :navigation_minimap, :boolean, default: true
    end
  end
end
