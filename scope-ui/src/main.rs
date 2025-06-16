use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay,
    SimulatorEvent, Window,
};
use embedded_hal::digital::OutputPin;
use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    SpidevDevice,
};
use rppal::{
    gpio::{self, Gpio},
    hal::Delay,
};
use scope_ui::{
    display::ili9341::{DisplaySize320x240, Ili9341},
    input::{InputEvent, MenuInput},
    menu::{MenuEvent, ScopeMenu},
    SPIInterface,
};

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
const ROTARY_CLK: u8 = 17;
const ROTARY_DT: u8 = 18;
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
    let mut display = Ili9341::new(interface, rst_pin, &mut Delay, DisplaySize320x240).unwrap();

    let rotary_clk = gpio.get(ROTARY_CLK).unwrap().into_output();
    let rotary_dt = gpio.get(ROTARY_DT).unwrap().into_output();

    // let output_settings = OutputSettingsBuilder::new()
    //     .theme(BinaryColorTheme::OledBlue)
    //     .build();
    // let mut input = InputWindow::new(&output_settings);
    // let mut display = SimulatorDisplay::new(Size::new(320, 240));
    // input.window.update(&display);
    let mut input = RotaryEncoder::new(rotary_clk, rotary_dt);
    let mut menu = ScopeMenu;
    menu.run(&mut display, &mut input);
}

pub struct RotaryEncoder<CLK, DT>
where
    CLK: OutputPin,
    DT: OutputPin,
{
    clk: CLK,
    dt: DT,
    // sw: Option<OutputPin>,
}

impl<CLK, DT> RotaryEncoder<CLK, DT>
where
    CLK: OutputPin,
    DT: OutputPin,
{
    pub fn new(clk: CLK, dt: DT) -> Self {
        Self { clk, dt }
    }
}

impl<CLK, DT> MenuInput for RotaryEncoder<CLK, DT>
where
    CLK: OutputPin,
    DT: OutputPin,
{
    fn poll(&mut self) -> Option<InputEvent> {
        // Implement rotary encoder logic here
        None
    }
}

pub struct InputWindow {
    window: Window,
}

impl InputWindow {
    pub fn new(output_settings: &OutputSettings) -> Self {
        let window = Window::new("Menu Simulation", output_settings);
        Self { window }
    }

    pub fn update(&mut self, display: &SimulatorDisplay<BinaryColor>) {
        self.window.update(display);
    }
}

// impl MenuInput for InputWindow {
//     fn poll(&mut self) -> Option<InputEvent> {
//         for event in self.window.events() {
//             match event {
//                 SimulatorEvent::Quit => return Some(InputEvent::Quit),
//                 SimulatorEvent::KeyDown { keycode, .. } => match keycode {
//                     Keycode::Up => return Some(InputEvent::Up),
//                     Keycode::Down => return Some(InputEvent::Down),
//                     Keycode::Return | Keycode::Space => return Some(InputEvent::Select),
//                     _ => return None,
//                 },
//                 _ => return None,
//             }
//         }
//         None
//     }
//
//     fn update(&mut self, display: &mut Self::Display) {
//         self.window.update(display);
//     }
// }

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
