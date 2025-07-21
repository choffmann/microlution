use std::{fmt::Debug, sync::mpsc, time::Duration};

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20, iso_8859_3::FONT_9X18_BOLD},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use embedded_layout::{
    align::{Align, horizontal, vertical},
    layout::linear::{FixedMargin, LinearLayout},
    prelude::*,
};
use linux_embedded_hal::{
    Delay, SpidevDevice,
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
};
use log::{debug, error};
use rppal::gpio::Gpio;
use scope_ui::{
    client::{AppClient, AppConfig, OpenflexureAxis},
    display::{
        Flushable,
        ili9341::{DisplaySize240x320, Ili9341, Orientation},
    },
    input::{MenuInput, rotary_encoder::RotaryEncoder},
};

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
const ROTARY_CLK: u8 = 17;
const ROTARY_DT: u8 = 18;
const ROTARY_SW: u8 = 27;

#[tokio::main]
async fn main() {
    env_logger::init();
    let gpio = Gpio::new().expect("Failed to setup gpio");
    let spidev = create_spi().expect("Failed to setup spi device");
    let spi = SpidevDevice(spidev);
    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();

    let rotary_clk = gpio.get(ROTARY_CLK).expect("Invalid CLK pin").into_input();
    let rotary_dt = gpio.get(ROTARY_DT).expect("Invalid DT pin").into_input();
    let rotary_sw = gpio.get(ROTARY_SW).expect("Invalid SW pin").into_input();

    let mut input = RotaryEncoder::new(rotary_clk, rotary_dt, rotary_sw);

    let config = AppConfig {
        openflexure_url: "http://localhost:5000".try_into().unwrap(),
        phoenix_url: "http://localhost:4000".try_into().unwrap(),
    };

    let iface = SPIInterface::new(spi, dc_pin);
    let display = Ili9341::new(
        iface,
        rst_pin,
        &mut Delay,
        Orientation::LandscapeFlipped,
        DisplaySize240x320,
    )
    .unwrap();

    let mut app = App::new(&config, display);

    app.clear();
    app.splash_screen(Rgb565::CSS_ORANGE);
    app.setup().await;
    std::thread::sleep(Duration::from_secs(3));
    app.clear();
    app.draw().unwrap();

    // run poll input in other thread
    let (event_tx, event_rx) = mpsc::channel();
    std::thread::spawn(move || {
        loop {
            if let Some(event) = input.poll() {
                event_tx.send(event).unwrap();
            }
        }
    });

    loop {
        if let Ok(event) = event_rx.recv() {
            debug!("receive event {:?}", event);
            match event {
                scope_ui::input::InputEvent::Up => app.increase().await,
                scope_ui::input::InputEvent::Down => app.decrease().await,
                scope_ui::input::InputEvent::Select => app.trigger_control_mode(),
                scope_ui::input::InputEvent::Quit => {}
            }
            app.clear();
        }

        app.draw().unwrap();
        // app.flush().unwrap();
    }
}

fn create_spi() -> Result<Spidev, std::io::Error> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(0x07735940)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;

    Ok(spi)
}

struct MenuSelection {
    name: &'static str,
    value: i64,
}

impl MenuSelection {
    fn new(name: &'static str, value: i64) -> Self {
        Self { name, value }
    }
}

struct App<D>
where
    D: DrawTarget<Color = Rgb565, Error: Debug> + Flushable,
{
    client: AppClient,
    display: D,
    selection_idx: u32,
    selections: Box<[MenuSelection]>,
    contol_mode: bool,
}

impl<D> Drop for App<D>
where
    D: DrawTarget<Color = Rgb565, Error: Debug> + Flushable,
{
    fn drop(&mut self) {
        self.clear();
        self.splash_screen(Rgb565::CSS_GRAY);
    }
}

impl<D> App<D>
where
    D: DrawTarget<Color = Rgb565, Error: Debug> + Flushable,
{
    pub fn new(config: &AppConfig, display: D) -> Self {
        let client = AppClient::new(config);
        let selections = [
            MenuSelection::new("X Axis", 0),
            MenuSelection::new("Y Axis", 0),
            MenuSelection::new("Z Axis", 0),
            MenuSelection::new("Slider", 0),
        ];
        Self {
            client,
            display,
            selections: Box::new(selections),
            selection_idx: 0,
            contol_mode: false,
        }
    }

    pub async fn setup(&mut self) {
        let flexure_values = self
            .client
            .get_openflexure_position()
            .await
            .unwrap_or_default();

        self.selections[0].value = flexure_values.x;
        self.selections[1].value = flexure_values.y;
        self.selections[2].value = flexure_values.z;
    }
}

impl<D> App<D>
where
    D: DrawTarget<Color = Rgb565, Error: Debug> + Flushable,
{
    pub fn draw(&mut self) -> anyhow::Result<()> {
        let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);
        self.display
            .bounding_box()
            .into_styled(thick_stroke)
            .draw(&mut self.display)
            .unwrap();

        self.draw_menu()
    }

    fn trigger_control_mode(&mut self) {
        self.contol_mode = !self.contol_mode;
        debug!("switch control mode to {}", self.contol_mode);
    }

    fn draw_menu(&mut self) -> anyhow::Result<()> {
        let display_area = self.display.bounding_box();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::WHITE)
            .build();

        let selector_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::CSS_ORANGE)
            .build();

        let selector_style_invisible = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::BLACK)
            .build();

        let control_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(if self.contol_mode {
                Rgb565::CSS_ORANGE
            } else {
                Rgb565::CSS_GRAY
            })
            .build();

        let mut selector = Vec::with_capacity(3);
        match dbg!(self.selection_idx) {
            0 => {
                selector.push(Text::new(">", Point::zero(), selector_style));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
            }
            1 => {
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
            }
            2 => {
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
            }
            3 => {
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style));
            }
            _ => {
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
                selector.push(Text::new(">", Point::zero(), selector_style_invisible));
            }
        }
        let x_axis = LinearLayout::horizontal(Chain::new(selector[0]).append(Text::new(
            self.selections[0].name,
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let y_axis = LinearLayout::horizontal(Chain::new(selector[1]).append(Text::new(
            self.selections[1].name,
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let z_axis = LinearLayout::horizontal(Chain::new(selector[2]).append(Text::new(
            self.selections[2].name,
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let slider = LinearLayout::horizontal(Chain::new(selector[3]).append(Text::new(
            self.selections[3].name,
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let control_txt = format!("Control Mode: {}", self.contol_mode);
        let control = Text::new(&control_txt, Point::zero(), control_style);

        LinearLayout::vertical(
            Chain::new(
                LinearLayout::horizontal(Chain::new(x_axis).append(Text::new(
                    format!("{}", self.selections[0].value).as_str(),
                    Point::zero(),
                    text_style,
                )))
                .with_spacing(FixedMargin(64))
                .arrange(),
            )
            .append(
                LinearLayout::horizontal(Chain::new(y_axis).append(Text::new(
                    format!("{}", self.selections[1].value).as_str(),
                    Point::zero(),
                    text_style,
                )))
                .with_spacing(FixedMargin(64))
                .arrange(),
            )
            .append(
                LinearLayout::horizontal(Chain::new(z_axis).append(Text::new(
                    format!("{}", self.selections[2].value).as_str(),
                    Point::zero(),
                    text_style,
                )))
                .with_spacing(FixedMargin(64))
                .arrange(),
            )
            .append(
                LinearLayout::horizontal(Chain::new(slider).append(Text::new(
                    "<   >",
                    Point::zero(),
                    text_style,
                )))
                .with_spacing(FixedMargin(64))
                .arrange(),
            )
            .append(control),
        )
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut self.display)
        .unwrap();

        Ok(())
    }

    pub fn splash_screen(&mut self, color: Rgb565) {
        let display_area = self.display.bounding_box();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::WHITE)
            .build();

        let border_style = PrimitiveStyleBuilder::new().fill_color(color).build();

        let text = Text::new("Microlution", Point::zero(), text_style);
        let border = Rectangle::new(Point::zero(), Size::new(text.size().width, 4))
            .into_styled(border_style);

        LinearLayout::vertical(Chain::new(text).append(border))
            .with_spacing(FixedMargin(3))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(&mut self.display)
            .unwrap();
    }

    pub async fn increase(&mut self) {
        if !self.contol_mode {
            self.selection_idx = self.selection_idx.wrapping_add(1) % self.selections.len() as u32;
        } else {
            let axis = match self.selection_idx {
                0 => OpenflexureAxis::X,
                1 => OpenflexureAxis::Y,
                2 => OpenflexureAxis::Z,
                3 => {
                    let _response = self
                        .client
                        .move_slider(true)
                        .await
                        .map_err(|e| error!("failed to move slider {:?}", e));

                    return;
                }
                _ => return,
            };
            let _ = self
                .client
                .move_openflexure(scope_ui::client::MoveDirection::Pos(axis))
                .await;
        }

        // update state
        let _ = self.setup().await;
    }

    pub async fn decrease(&mut self) {
        if !self.contol_mode {
            self.selection_idx = self.selection_idx.wrapping_sub(1) % self.selections.len() as u32;
        } else {
            let axis = match self.selection_idx {
                0 => OpenflexureAxis::X,
                1 => OpenflexureAxis::Y,
                2 => OpenflexureAxis::Z,
                3 => {
                    let _ = self
                        .client
                        .move_slider(false)
                        .await
                        .map_err(|e| error!("failed to move slider {:?}", e));
                    return;
                }
                _ => return,
            };
            let _ = self
                .client
                .move_openflexure(scope_ui::client::MoveDirection::Neg(axis))
                .await;

            // update state
            let _ = self.setup().await;
        }
    }

    // pub fn flush(&mut self) -> anyhow::Result<()> {
    //     self.display.flush().unwrap();
    //     Ok(())
    // }

    pub fn clear(&mut self) {
        // self.display.clear(BinaryColor::Off).unwrap();
        self.display.clear(Rgb565::BLACK).unwrap();
    }
}
