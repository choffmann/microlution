use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    SpidevDevice,
};
use rppal::{gpio::Gpio, hal::Delay};
use scope_ui::{
    client::AppConfig,
    display::ili9341::{DisplaySize320x240, Ili9341},
    input::rotary_encoder::RotaryEncoder,
    menu::ScopeMenu,
    SPIInterface,
};

const DC_PIN: u8 = 24;
const RST_PIN: u8 = 25;
const CS_PIN: u8 = 8;
const ROTARY_CLK: u8 = 18;
const ROTARY_DT: u8 = 17;
const ROTARY_SW: u8 = 27;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let gpio = Gpio::new().expect("Failed to setup gpio");
    let spidev = create_spi().expect("Failed to setup spi device");
    let spi = SpidevDevice(spidev);
    let mut cs_pin = gpio.get(CS_PIN).unwrap().into_output();
    cs_pin.set_low();
    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let interface = SPIInterface::new(spi, dc_pin);
    let display = Ili9341::new(interface, rst_pin, &mut Delay, DisplaySize320x240).unwrap();

    let rotary_clk = gpio.get(ROTARY_CLK).expect("Invalid CLK pin").into_input();
    let rotary_dt = gpio.get(ROTARY_DT).expect("Invalid DT pin").into_input();
    let rotary_sw = gpio.get(ROTARY_SW).expect("Invalid SW pin").into_input();

    let mut input = RotaryEncoder::new(rotary_clk, rotary_dt, rotary_sw);
    let config = AppConfig {
        openflexure_url: "http://localhost:5000".try_into().unwrap(),
        phoenix_url: "http://localhost:4000".try_into().unwrap(),
    };

    let mut menu = ScopeMenu::new(&config, display);
    menu.run(&mut input).await
}

// pub struct InputWindow {
//     window: Window,
// }
//
// impl InputWindow {
//     pub fn new(output_settings: &OutputSettings) -> Self {
//         let window = Window::new("Menu Simulation", output_settings);
//         Self { window }
//     }
//
//     pub fn update(&mut self, display: &SimulatorDisplay<BinaryColor>) {
//         self.window.update(display);
//     }
// }

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
