use display::{
    display::ili9341::{self, Ili9341},
    SPIInterface,
};
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    Delay, SpidevDevice,
};
use rppal::gpio::Gpio;

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
// const ROTARY_CLK: u8 = 17;
// const ROTARY_DT: u8 = 18;
// const ROTARY_SW: u8 = 27;

pub const DISP_WIDTH: u32 = 160;
pub const DISP_HEIGHT: u32 = 128;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup gpio");
    let spidev = create_spi().expect("Failed to setup spi device");
    let spi = SpidevDevice(spidev);
    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let interface = SPIInterface::new(spi, dc_pin);
    let mut display =
        Ili9341::new(interface, rst_pin, &mut Delay, ili9341::DisplaySize240x320).unwrap();

    display.clear_screen(0x00).unwrap();
    Delay.delay_ms(1000);
    display.clear_screen(0x13).unwrap();
    Delay.delay_ms(1000);
    display.clear_screen(0x00).unwrap();

    if let Ok(status) = display.status() {
        println!("status: {:2x?}", status);
    }
}

#[cfg(feature = "simulator")]
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

fn create_spi() -> Result<Spidev, std::io::Error> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        // .max_speed_hz(20_000)
        .max_speed_hz(0x07735940)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}
