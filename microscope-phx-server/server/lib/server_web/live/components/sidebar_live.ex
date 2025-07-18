defmodule ServerWeb.SidebarLive do
  use ServerWeb, :live_component

  alias Server.NavConst
  alias Server.Api

  def render(assigns) do
    ~H"""
    <aside id="sidebar">
      <div class="sidebarz shadow">
        <div class="links">
          <p class="h3 text-center">MICROLUTION</p>
          <hr />
          <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="View"
            title="View"
            href={NavConst.view()}
            icon="bi-eye"
          />
          <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Gallery"
            title="Gallery"
            href={NavConst.gallery()}
            icon="bi-images fs-2"
          />
          <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Navigate"
            title="Navigate"
            href={NavConst.navigate()}
            icon="bi-dpad-fill fs-2"
          />
          <%!-- <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Capture"
            title="Capture"
            href={NavConst.capture()}
            icon="bi-camera-fill fs-2"
          /> --%>
          <%!-- <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Automatic"
            title="Automatic"
            href={NavConst.automatic()}
            icon="bi-radioactive fs-2"
          /> --%>
          <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Stitching"
            title="Stitching"
            href={NavConst.stitching()}
            icon="bi-x-diamond fs-2"
          />
                    <.sidebar_item
            class=""
            menu_active={@menu_active}
            menu_name="Stitching Inspect"
            title="Stitching Inspect"
            href={NavConst.stitching_inspect()}
            icon="bi-x-diamond fs-2"
          />
                    <%!-- <.sidebar_a
            class=""
            menu_active={@menu_active}
            menu_name="OpenFlexure"
            title="OpenFlexure"
            href={Api.open_flexure_site()}
            icon="bi-x-diamond fs-2"
          /> --%>
        </div>
      </div>
    </aside>
    """
  end

  def mount(socket) do
    active =
      case socket.view do
        ServerWeb.IndexLive -> "Index"
        ServerWeb.ViewLive -> "View"
        ServerWeb.GalleryLive -> "Gallery"
        ServerWeb.NavigateLive -> "Navigate"
        ServerWeb.CaptureLive -> "Capture"
        ServerWeb.AutomaticLive -> "Automatic"
        ServerWeb.StitchingLive -> "Stitching"
        ServerWeb.StitchingInspectLive -> "Stitching Inspect"
        ServerWeb.StorageLive -> "Storage"
        ServerWeb.SettingsLive -> "Settings"
        ServerWeb.LoggingLive -> "Logging"
        ServerWeb.AboutLive -> "About"
        _ -> "Index"
      end

    socket =
      socket
      |> assign(:menu_active, active)

    {:ok, socket}
  end

  def sidebar_item(assigns) do
    ~H"""
    <.link
      navigate={@href}
      class={"#{@class} #{if @menu_active == @menu_name do "active" end}"}
      title=""
    >
      <span class={"#{@icon} sidebar-item-icon"}></span>
      <h5>{@title}</h5>
    </.link>
    """
  end

  def sidebar_a(assigns) do
    ~H"""
    <a
      href={@href}
      class={"#{@class} #{if @menu_active == @menu_name do "active" end}"}
      title=""
    >
      <span class={"#{@icon} sidebar-item-icon"}></span>
      <h5>{@title}</h5>
    </a>
    """
  end
end
