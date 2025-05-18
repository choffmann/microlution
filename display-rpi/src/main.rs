use std::{thread, time::Duration};

use display::ui::{Menu, MenuItem};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_hal::{delay::DelayNs, digital::OutputPin, spi};
use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    Delay, SpidevDevice,
};
use rotary_encoder_embedded::{Direction, RotaryEncoder};
use rppal::gpio::Gpio;
use st7735_lcd::{Orientation, ST7735};

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
const ROTARY_CLK: u8 = 17;
const ROTARY_DT: u8 = 18;
const ROTARY_SW: u8 = 27;

pub const DISP_WIDTH: u32 = 160;
pub const DISP_HEIGHT: u32 = 128;

fn main() {
    let items = vec![
        MenuItem {
            title: "Control",
            selected: true,
        },
        MenuItem {
            title: "Scan",
            selected: false,
        },
        MenuItem {
            title: "Settings",
            selected: false,
        },
        MenuItem {
            title: "Info",
            selected: false,
        },
    ];

    let mut menu = Menu::new(items);

    // emulate(&mut menu);
    st7735(&mut menu);
}

fn st7735(menu: &mut Menu) {
    let gpio = Gpio::new().expect("Failed to setup gpio");
    let spidev = create_spi().expect("Failed to setup spi device");
    let spi = SpidevDevice(spidev);

    let rotary_dt = gpio.get(ROTARY_DT).unwrap().into_input();
    let rotary_clk = gpio.get(ROTARY_CLK).unwrap().into_input();
    let mut rotary_encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let mut display = setup_st7735(spi, dc_pin, rst_pin);
    menu.draw(&mut display).unwrap();

    loop {
        match rotary_encoder.update() {
            Direction::Clockwise => {
                menu.next_item().unwrap();
                menu.draw(&mut display).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
            Direction::Anticlockwise => {
                menu.prev_item().unwrap();
                menu.draw(&mut display).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
            Direction::None => {}
        }

        thread::sleep(Duration::from_millis(2));
    }
}

fn emulate(menu: &mut Menu) {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(DISP_WIDTH, DISP_HEIGHT));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Hello World", &output_settings);
    menu.draw(&mut display).unwrap();

    'running: loop {
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,

                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Left => {
                            menu.prev_item().unwrap();
                            menu.draw(&mut display).unwrap();
                        }
                        Keycode::Right => {
                            menu.next_item().unwrap();
                            menu.draw(&mut display).unwrap();
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        }
    }
}

fn setup_st7735<SPI, DC, RST>(spi: SPI, dc: DC, rst: RST) -> ST7735<SPI, DC, RST>
where
    SPI: spi::SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
{
    let mut display = ST7735::new(spi, dc, rst, true, false, DISP_WIDTH, DISP_HEIGHT);
    display.init(&mut Delay).expect("Failed to init display");

    display.set_orientation(&Orientation::Landscape).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    display
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
