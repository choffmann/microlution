defmodule Server.Repo.Migrations.AddShowMinimapBoundaries do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :show_mm_features, :boolean, default: true
    end
  end
end
