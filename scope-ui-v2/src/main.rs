use std::{fmt::Debug, time::Duration};

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_6X9, FONT_10X20},
        iso_8859_3::FONT_9X18_BOLD,
    },
    pixelcolor::{BinaryColor, Rgb565, raw::RawU16},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Text, renderer::CharacterStyle},
};
use embedded_layout::{
    align::{Align, horizontal, vertical},
    layout::linear::{FixedMargin, LinearLayout, spacing::DistributeFill},
    prelude::*,
};
use linux_embedded_hal::{
    Delay, SpidevDevice,
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
};
use rppal::gpio::Gpio;
use scope_ui::{
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

fn main() {
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

    let iface = SPIInterface::new(spi, dc_pin);
    let display = Ili9341::new(
        iface,
        rst_pin,
        &mut Delay,
        Orientation::LandscapeFlipped,
        DisplaySize240x320,
    )
    .unwrap();

    let mut app = App::new(display);

    app.clear();
    app.startup();
    std::thread::sleep(Duration::from_secs(3));
    app.clear();

    loop {
        if let Some(event) = input.poll() {
            match event {
                scope_ui::input::InputEvent::Up => app.increase(),
                scope_ui::input::InputEvent::Down => app.decrease(),
                scope_ui::input::InputEvent::Select => {}
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

struct App<D> {
    display: D,
    counter: u32,
    selection_idx: u32,
    contol_mode: bool,
}

impl<D> App<D>
where
    D: DrawTarget,
{
    pub fn new(display: D) -> Self {
        Self {
            display,
            counter: 0,
            selection_idx: 0,
            contol_mode: false,
        }
    }
}

// impl<D> App<D>
// where
//     D: DrawTarget<Color = Rgb565, Error: Debug>,
// {
//     pub fn draw(&mut self, point: Point) -> anyhow::Result<()> {
//         //let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//         let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::WHITE);
//         Text::new("Hello, World!", point, text_style)
//             .draw(&mut self.display)
//             .unwrap();
//
//         Ok(())
//     }
//
//     pub fn clear(&mut self) {
//         self.display.clear(Rgb565::BLACK).unwrap();
//     }
// }

impl<D> App<D>
where
    D: DrawTarget<Color = Rgb565, Error: Debug> + Flushable,
{
    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_menu()
    }

    fn draw_menu(&mut self) -> anyhow::Result<()> {
        let display_area = self.display.bounding_box();
        let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);

        display_area
            .into_styled(thick_stroke)
            .draw(&mut self.display)
            .unwrap();

        let text_style_selected = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::BLACK)
            .background_color(Rgb565::WHITE)
            .build();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::WHITE)
            .build();

        let selector_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(Rgb565::CSS_ORANGE)
            .build();

        // let text_style_backgound = MonoTextStyle::new(&FONT_9X18_BOLD, BinaryColor::Off)
        //     .set_background_color(Some(BinaryColor::On));

        let selected = self.selection_idx % 3;
        let selector = Text::new(">", Point::zero(), selector_style);
        let x_axis = LinearLayout::horizontal(Chain::new(selector).append(Text::new(
            "X Axis",
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let y_axis = LinearLayout::horizontal(Chain::new(selector).append(Text::new(
            "Y Axis",
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        let z_axis = LinearLayout::horizontal(Chain::new(selector).append(Text::new(
            "Z Axis",
            Point::zero(),
            text_style,
        )))
        .with_spacing(FixedMargin(5))
        .arrange();

        LinearLayout::vertical(Chain::new(x_axis).append(y_axis).append(z_axis))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(&mut self.display)
            .unwrap();

        // Text::with_alignment(
        //     "Z Axis",
        //     display_area.center() - Point::new(0, 15),
        //     text_style,
        //     embedded_graphics::text::Alignment::Center,
        // )
        // .draw(&mut self.display)
        // .unwrap();

        // LinearLayout::vertical(Chain::new(x_axis).append(y_axis).append(z_axis))
        //     .with_alignment(horizontal::Center)
        //     .arrange()
        //     .align_to(&display_area, horizontal::Center, vertical::Center)
        //     .draw(&mut self.display)
        //     .unwrap();

        Ok(())
    }

    pub fn startup(&mut self) {
        let display_area = self.display.bounding_box();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::WHITE)
            .build();

        let border_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::CSS_ORANGE)
            .build();

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

    pub fn increase(&mut self) {
        self.counter = self.counter.wrapping_add(1);
        self.selection_idx = self.counter.wrapping_add(1);
    }

    pub fn decrease(&mut self) {
        self.counter = self.counter.wrapping_sub(1);
        self.selection_idx = self.counter.wrapping_sub(1);
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.display.flush().unwrap();
        Ok(())
    }

    pub fn clear(&mut self) {
        // self.display.clear(BinaryColor::Off).unwrap();
        self.display.clear(Rgb565::BLACK).unwrap();
    }
}
