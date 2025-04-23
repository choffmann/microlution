use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    Delay, SpidevDevice,
};
use rppal::gpio::Gpio;
use st7735_lcd::{Orientation, ST7735};

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
const BL_PIN: u8 = 36;

const DISP_WIDTH: u32 = 160;
const DISP_HEIGHT: u32 = 128;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup gpio");
    let spidev = create_spi().expect("Failed to setup spi device");
    let spi = SpidevDevice(spidev);

    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let mut bl_pin = gpio.get(BL_PIN).unwrap().into_output();
    bl_pin.set_high(); // Backlight

    let mut display = ST7735::new(spi, dc_pin, rst_pin, true, false, DISP_WIDTH, DISP_HEIGHT);
    display.init(&mut Delay).expect("Failed to init display");

    display.set_orientation(&Orientation::Landscape).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    let mut old_value = 0;
    loop {
        for i in 0..=100 {
            draw_progress_bar(&mut display, i, old_value);
            old_value = i;
            Delay.delay_ms(50);
        }

        display.clear(Rgb565::BLACK).unwrap();
    }
}

fn create_spi() -> Result<Spidev, std::io::Error> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

fn draw_progress_bar<D>(display: &mut D, value: u8, old_value: u8)
where
    D: DrawTarget<Color = Rgb565>,
    D::Error: core::fmt::Debug,
{
    const BAR_WIDTH: i32 = 100;
    const BAR_HEIGHT: i32 = 10;
    const BAR_X: i32 = 3;
    const BAR_Y: i32 = 10;
    const TEXT_Y_OFFSET: i32 = 18;
    const TEXT_WIDTH: u32 = 6;

    let filled_width = (BAR_WIDTH * value as i32) / 100;
    let old_filled_width = (BAR_WIDTH * old_value as i32) / 100;

    if filled_width > old_filled_width {
        let new_rect = Rectangle::new(
            Point::new(BAR_X + old_filled_width, BAR_Y),
            Size::new((filled_width - old_filled_width) as u32, BAR_HEIGHT as u32),
        );
        new_rect
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .draw(display)
            .unwrap();
    }

    let text_position = Point::new(BAR_X, BAR_Y + TEXT_Y_OFFSET);
    let clear_rect = if value.to_string().len() > old_value.to_string().len() {
        Rectangle::new(
            Point::new(BAR_X + 77, BAR_Y + TEXT_Y_OFFSET - 7),
            Size::new(TEXT_WIDTH * 7 as u32, 9),
        )
    } else {
        Rectangle::new(
            Point::new(BAR_X + 77, BAR_Y + TEXT_Y_OFFSET - 7),
            Size::new(TEXT_WIDTH * value.to_string().len() as u32, 9),
        )
    };

    clear_rect
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)
        .unwrap();

    let hashes = "#".repeat((value / 10) as usize);
    let spaces = " ".repeat(10 - (value / 10) as usize);
    let text = format!("[{}{}] {}/100", hashes, spaces, value);

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::GREEN);
    Text::new(&text, text_position, style)
        .draw(display)
        .unwrap();
}
