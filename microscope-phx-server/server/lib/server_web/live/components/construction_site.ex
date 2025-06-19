defmodule ServerWeb.Components.ConstructionSite do
  use ServerWeb, :live_component

  def render(assigns) do
    ~H"""
    <div class="h-100 w-100 d-flex flex-column justify-content-center">
      <p class="h1 text-center">MICROLUTION</p>

      <p class="h3 text-center mb-5">under construction...</p>
      <img src="/images/baustelle.gif" class="w-50 h-50 ml-auto mr-auto" alt="" />
    </div>
    """
  end
end
