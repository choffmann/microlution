use std::{process::exit, time::Duration};

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
use log::info;
use position::Position;

use crate::input::{InputEvent, MenuInput};

mod position;

macro_rules! draw_menu {
    ($menu:expr, $display:expr, $name:expr) => {
        $menu.update($display);
        if let Err(_e) = $menu.draw($display) {
            eprintln!("Failed to draw {} menu", $name);
        }
    };
}

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

pub struct ScopeMenu;

impl ScopeMenu {
    pub fn run<D, I>(&mut self, display: &mut D, input: &mut I)
    where
        D: DrawTarget<Color = Rgb565>,
        I: MenuInput,
    {
        let mut state: MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line> =
            Default::default();
        let mut data = MenuData::default();

        loop {
            match data.current_view {
                MenuView::MainMenu => main_menu(display, input, &mut state, &mut data),
                MenuView::Control => control_menu(display, input, &mut state, &mut data),
                MenuView::Scan | MenuView::Settings | MenuView::Info => {
                    println!("{:?} view is not implemented yet.", data.current_view);
                    data.current_view = MenuView::MainMenu;
                }
            }

            std::thread::sleep(Duration::from_micros(1000));
        }
    }
}

fn try_clear_display<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    if let Err(_e) = display.clear(Rgb565::BLACK) {
        eprintln!("Failed to clear display");
    }
}

fn update_position(pos: &mut Position, dir: ControlAction) {
    let value = pos.value();
    if value == 0 && matches!(dir, ControlAction::Decrease) {
        return;
    }
    match dir {
        ControlAction::Increase => pos.set_value(value.wrapping_add(1)),
        ControlAction::Decrease => pos.set_value(value.wrapping_sub(1)),
        ControlAction::NoAction => {}
    }
}

pub fn handle_event(event: MenuEvent, data: &mut MenuData) {
    match event {
        MenuEvent::Navigate(view) => {
            info!("switching to view {:?}", view);
            data.current_view = view
        }
        MenuEvent::ControlMode(mode) => match mode {
            ScopeControlMode::SampleChanger(dir) => {
                info!(
                    "control sample changer. pos: {:?}, dir: {:?}",
                    data.sample_changer_pos, dir
                );
                update_position(&mut data.sample_changer_pos, dir);
            }
            ScopeControlMode::Microscope(axis, dir) => {
                let pos = match axis {
                    MicroscopeAxis::X => &mut data.microscope_x_pos,
                    MicroscopeAxis::Y => &mut data.microscope_y_pos,
                    MicroscopeAxis::Z => &mut data.microscope_z_pos,
                };
                info!(
                    "control sample changer. pos: {:?}, dir: {:?}",
                    data.sample_changer_pos, dir
                );
                update_position(pos, dir);
            }
        },
        MenuEvent::InputLock(v) => {
            info!("Lock input. scope: {:?}", v);
            data.lock_input = Some(v)
        }
        MenuEvent::InputUnlock => {
            info!("Unlock input");
            data.lock_input = None
        }
        MenuEvent::Quit => exit(0),
        MenuEvent::Nothing => {}
    }
}

pub fn main_menu<D, I>(
    display: &mut D,
    input: &mut I,
    state: &mut MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line>,
    data: &mut MenuData,
) where
    D: DrawTarget<Color = Rgb565>,
    I: MenuInput,
{
    let main_menu_items = vec![
        MenuItem::new("Control", ">")
            .with_value_converter(|_| MenuEvent::Navigate(MenuView::Control)),
        MenuItem::new("Scan", ">").with_value_converter(|_| MenuEvent::Navigate(MenuView::Scan)),
        MenuItem::new("Settings", ">")
            .with_value_converter(|_| MenuEvent::Navigate(MenuView::Settings)),
        MenuItem::new("Info", ">").with_value_converter(|_| MenuEvent::Navigate(MenuView::Info)),
    ];

    let mut menu = Menu::with_style(
        "Microlution",
        MenuStyle::new(MenuTheme)
            .with_font(&FONT_8X13)
            .with_title_font(&FONT_10X20),
    )
    .add_menu_items(main_menu_items)
    .build_with_state(*state);

    draw_menu!(&mut menu, display, "main");
    // input.update(display);

    let event = match input.poll() {
        Some(InputEvent::Up) => menu.interact(Interaction::Navigation(Navigation::Previous)),
        Some(InputEvent::Down) => menu.interact(Interaction::Navigation(Navigation::Next)),
        Some(InputEvent::Select) => menu.interact(Interaction::Action(Action::Select)),
        Some(InputEvent::Quit) => Some(MenuEvent::Quit),
        None => None,
    };

    if let Some(event) = event {
        try_clear_display(display);
        handle_event(event, data);
    }

    *state = menu.state();
}

fn control_menu<D, I>(
    display: &mut D,
    input: &mut I,
    state: &mut MenuState<ProgrammedAdapter<MenuEvent>, StaticPosition, Line>,
    data: &mut MenuData,
) where
    D: DrawTarget<Color = Rgb565>,
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

    draw_menu!(&mut menu, display, "control");
    // input.update(display);

    let event = match input.poll() {
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
        try_clear_display(display);
        handle_event(event, data);
    }

    *state = menu.state();
}
