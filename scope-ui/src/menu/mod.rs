use std::{fmt::Debug, process::exit, time::Duration};

use embedded_graphics::{
    mono_font::ascii::{FONT_10X20, FONT_8X13},
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_menu::{
    interaction::{programmed::ProgrammedAdapter, Action, Interaction, Navigation},
    items::MenuItem,
    selection_indicator::{style::Line, StaticPosition},
    theme::Theme,
    Menu, MenuState, MenuStyle,
};
use log::{debug, error};
use position::Position;

use crate::{
    client::{AppClient, AppConfig, OpenflexureAxis},
    input::{InputEvent, MenuInput},
};

mod position;

#[derive(Clone, Copy)]
struct MenuTheme;

impl Theme for MenuTheme {
    type Color = Rgb565;

    fn text_color(&self) -> Self::Color {
        Rgb565::WHITE
    }

    fn selected_text_color(&self) -> Self::Color {
        Rgb565::BLACK
    }

    fn selection_color(&self) -> Self::Color {
        Rgb565::new(51, 255, 51)
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum MenuView {
    #[default]
    MainMenu,
    Control,
    Scan,
    Settings,
    Info,
}

#[derive(Clone, Copy, Debug)]
pub enum MicroscopeAxis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
pub enum ControlAction {
    Increase,
    Decrease,
    NoAction,
}

#[derive(Clone, Debug)]
pub enum ScopeControlMode {
    SampleChanger(ControlAction),
    Microscope(MicroscopeAxis, ControlAction),
}

#[derive(Default, Clone, Debug)]
pub enum MenuEvent {
    Navigate(MenuView),
    InputLock(ScopeControlMode),
    InputUnlock,
    ControlMode(ScopeControlMode),
    #[default]
    Nothing,
    Quit,
}

#[derive(Clone, Debug)]
pub struct MenuData {
    current_view: MenuView,
    lock_input: Option<ScopeControlMode>,
    sample_changer_pos: Position,
    microscope_x_pos: Position,
    microscope_y_pos: Position,
    microscope_z_pos: Position,
}

impl Default for MenuData {
    fn default() -> Self {
        Self {
            current_view: MenuView::MainMenu,
            lock_input: None,
            sample_changer_pos: Position::new(10),
            microscope_x_pos: Position::new(23),
            microscope_y_pos: Position::new(3329),
            microscope_z_pos: Position::new(338),
        }
    }
}

pub struct ScopeMenu {
    client: AppClient,
}

impl ScopeMenu {
    pub fn new(config: &AppConfig) -> Self {
        let client = AppClient::new(config);
        Self { client }
    }

    pub async fn run<D, I>(&mut self, display: &mut D, input: &mut I) -> anyhow::Result<()>
    where
        D: DrawTarget<Color = Rgb565, Error: Debug>,
        I: MenuInput,
    {
        let _ = &self.try_clear_display(display);

        // Show logo / text

        let flexure_stage = self.client.get_openflexure_position().await?;

        let mut state: MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line> =
            Default::default();
        let mut data = MenuData {
            current_view: MenuView::MainMenu,
            lock_input: None,
            sample_changer_pos: Position::new(10),
            microscope_x_pos: Position::new(flexure_stage.x),
            microscope_y_pos: Position::new(flexure_stage.y),
            microscope_z_pos: Position::new(flexure_stage.z),
        };

        loop {
            match data.current_view {
                MenuView::MainMenu => self.main_menu(display, input, &mut state, &mut data).await,
                MenuView::Control => {
                    self.control_menu(display, input, &mut state, &mut data)
                        .await
                }
                MenuView::Scan | MenuView::Settings | MenuView::Info => {
                    println!("{:?} view is not implemented yet.", data.current_view);
                    data.current_view = MenuView::MainMenu;
                }
            }

            std::thread::sleep(Duration::from_micros(1000));
        }
    }

    fn try_clear_display<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) {
        if let Err(_e) = display.clear(Rgb565::BLACK) {
            error!("Failed to clear display");
        }
    }

    fn update_position(&self, pos: &mut Position, dir: &ControlAction) {
        let value = pos.value();
        match dir {
            ControlAction::Increase => pos.set_value(value.wrapping_add(200)),
            ControlAction::Decrease => pos.set_value(value.wrapping_sub(200)),
            ControlAction::NoAction => {}
        }
    }

    async fn send_microscope_axis(&self, axis: &MicroscopeAxis, dir: &ControlAction) {
        let axis = match axis {
            MicroscopeAxis::X => OpenflexureAxis::X,
            MicroscopeAxis::Y => OpenflexureAxis::Y,
            MicroscopeAxis::Z => OpenflexureAxis::Z,
        };

        match dir {
            ControlAction::Increase => {
                let _ = self
                    .client
                    .move_openflexure(crate::client::MoveDirection::Pos(axis))
                    .await
                    .map_err(|e| error!("{e}"));
            }
            ControlAction::Decrease => {
                let _ = self
                    .client
                    .move_openflexure(crate::client::MoveDirection::Neg(axis))
                    .await
                    .map_err(|e| error!("{e}"));
            }
            ControlAction::NoAction => {}
        };
    }

    pub async fn handle_event(&self, event: MenuEvent, data: &mut MenuData) {
        match event {
            MenuEvent::Navigate(view) => {
                debug!("switching to view {:?}", view);
                data.current_view = view
            }
            MenuEvent::ControlMode(mode) => match mode {
                ScopeControlMode::SampleChanger(dir) => {
                    debug!(
                        "control sample changer. pos: {:?}, dir: {:?}",
                        data.sample_changer_pos, dir
                    );
                    self.update_position(&mut data.sample_changer_pos, &dir);
                }
                ScopeControlMode::Microscope(axis, dir) => {
                    let pos = match axis {
                        MicroscopeAxis::X => &mut data.microscope_x_pos,
                        MicroscopeAxis::Y => &mut data.microscope_y_pos,
                        MicroscopeAxis::Z => &mut data.microscope_z_pos,
                    };
                    debug!(
                        "control sample changer. pos: {:?}, dir: {:?}",
                        data.sample_changer_pos, dir
                    );
                    self.update_position(pos, &dir);
                    self.send_microscope_axis(&axis, &dir).await;
                }
            },
            MenuEvent::InputLock(v) => {
                debug!("Lock input. scope: {:?}", v);
                data.lock_input = Some(v)
            }
            MenuEvent::InputUnlock => {
                debug!("Unlock input");
                data.lock_input = None
            }
            MenuEvent::Quit => exit(0),
            MenuEvent::Nothing => {}
        }
    }

    pub async fn main_menu<D, I>(
        &self,
        display: &mut D,
        input: &mut I,
        state: &mut MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line>,
        data: &mut MenuData,
    ) where
        D: DrawTarget<Color = Rgb565, Error: Debug>,
        I: MenuInput,
    {
        let main_menu_items = vec![
            MenuItem::new("Control", ">")
                .with_value_converter(|_| MenuEvent::Navigate(MenuView::Control)),
            MenuItem::new("Scan", ">")
                .with_value_converter(|_| MenuEvent::Navigate(MenuView::Scan)),
            MenuItem::new("Settings", ">")
                .with_value_converter(|_| MenuEvent::Navigate(MenuView::Settings)),
            MenuItem::new("Info", ">")
                .with_value_converter(|_| MenuEvent::Navigate(MenuView::Info)),
        ];

        let mut menu = Menu::with_style(
            "Microlution",
            MenuStyle::new(MenuTheme)
                .with_font(&FONT_8X13)
                .with_title_font(&FONT_10X20),
        )
        .add_menu_items(main_menu_items)
        .build_with_state(*state);

        let event = input.poll();
        let event = match event {
            Some(InputEvent::Up) => menu.interact(Interaction::Navigation(Navigation::Previous)),
            Some(InputEvent::Down) => menu.interact(Interaction::Navigation(Navigation::Next)),
            Some(InputEvent::Select) => menu.interact(Interaction::Action(Action::Select)),
            Some(InputEvent::Quit) => Some(MenuEvent::Quit),
            None => None,
        };

        if let Some(event) = event {
            self.try_clear_display(display);
            self.handle_event(event, data).await;

            menu.update(display);
            menu.draw(display).unwrap();
        }

        *state = menu.state();
    }

    async fn control_menu<D, I>(
        &self,
        display: &mut D,
        input: &mut I,
        state: &mut MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line>,
        data: &mut MenuData,
    ) where
        D: DrawTarget<Color = Rgb565, Error: Debug>,
        I: MenuInput,
    {
        let mut menu = Menu::with_style(
            "Control",
            MenuStyle::new(MenuTheme)
                .with_font(&FONT_8X13)
                .with_title_font(&FONT_10X20),
        )
        .add_item("Back", "<<", |_| MenuEvent::Navigate(MenuView::MainMenu))
        .add_section_title("Sample Changer")
        .add_item("  Slider", data.sample_changer_pos.clone(), |_p| {
            MenuEvent::InputLock(ScopeControlMode::SampleChanger(ControlAction::NoAction))
        })
        .add_section_title("Microscope Control")
        .add_item("  X Axis", data.microscope_x_pos.clone(), |_p| {
            MenuEvent::InputLock(ScopeControlMode::Microscope(
                MicroscopeAxis::X,
                ControlAction::NoAction,
            ))
        })
        .add_item("  Y Axis", data.microscope_y_pos.clone(), |_p| {
            MenuEvent::InputLock(ScopeControlMode::Microscope(
                MicroscopeAxis::Y,
                ControlAction::NoAction,
            ))
        })
        .add_item("  Z Axis", data.microscope_z_pos.clone(), |_p| {
            MenuEvent::InputLock(ScopeControlMode::Microscope(
                MicroscopeAxis::Z,
                ControlAction::NoAction,
            ))
        })
        .build_with_state(*state);

        let event = input.poll();
        let event = match event {
            Some(InputEvent::Up) => {
                if let Some(lock) = &data.lock_input {
                    Some(match lock {
                        ScopeControlMode::SampleChanger(_) => MenuEvent::ControlMode(
                            ScopeControlMode::SampleChanger(ControlAction::Increase),
                        ),
                        ScopeControlMode::Microscope(axis, _) => MenuEvent::ControlMode(
                            ScopeControlMode::Microscope(*axis, ControlAction::Increase),
                        ),
                    })
                } else {
                    menu.interact(Interaction::Navigation(Navigation::Previous))
                }
            }
            Some(InputEvent::Down) => {
                if let Some(lock) = &data.lock_input {
                    Some(match lock {
                        ScopeControlMode::SampleChanger(_) => MenuEvent::ControlMode(
                            ScopeControlMode::SampleChanger(ControlAction::Decrease),
                        ),
                        ScopeControlMode::Microscope(axis, _) => MenuEvent::ControlMode(
                            ScopeControlMode::Microscope(*axis, ControlAction::Decrease),
                        ),
                    })
                } else {
                    menu.interact(Interaction::Navigation(Navigation::Next))
                }
            }
            Some(InputEvent::Select) => {
                if data.lock_input.is_some() {
                    Some(MenuEvent::InputUnlock)
                } else {
                    menu.interact(Interaction::Action(Action::Select))
                }
            }
            Some(InputEvent::Quit) => Some(MenuEvent::Quit),
            None => None,
        };

        if let Some(event) = event {
            let _ = &self.try_clear_display(display);
            let _ = &self.handle_event(event, data).await;

            menu.update(display);
            menu.draw(display).unwrap();
        }

        *state = menu.state();
    }
}
