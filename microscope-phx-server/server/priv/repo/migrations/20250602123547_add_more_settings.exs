defmodule Server.Repo.Migrations.AddMoreSettings do
  use Ecto.Migration

  def change do
    alter table(:settings) do
      add :show_mm, :boolean, default: false
      add :stream_mm, :boolean, default: false
    end
  end
end
