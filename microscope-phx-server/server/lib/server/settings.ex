defmodule Server.Settings do
  import Ecto.Query, warn: false

  alias Server.Repo
  alias Server.Settings.Setting

  def save(settings_params) do
    %Setting{}
    |> Setting.changeset(settings_params)
    |> Repo.insert()
  end

  def check_if_exists(id) do
    Repo.exists?(
      from(s in Setting,
        where: s.home_x == 0
      )
    )
    |> IO.inspect()
  end

  def get_settings!(id) do
    query =
      from(s in Setting,
        where: s.id == ^id,
        select: s
      )

    Repo.all(query)
    |> List.first()
  end

  def update(id, params) do
    settings = get_settings!(id)

    settings
    |> Setting.changeset(params)
    |> Repo.update()
  end
end
