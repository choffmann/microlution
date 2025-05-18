#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rotary_encoder_embedded::{Direction, RotaryEncoder};
// Provide an alias for our BSP so we can switch targets quickly.
use rp_pico::{self as bsp};

use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // let mut peripherals = pac::Peripherals::take().unwrap();
    //
    // let sclk = pins.gpio18.into_function::<FunctionSpi>();
    // let mosi = pins.gpio19.into_function::<FunctionSpi>();
    //
    // let spi_device = peripherals.SPI0;
    // let spi_pin_layout = (mosi, sclk);
    //
    // let spi = Spi::<_, _, _, 8>::new(spi_device, spi_pin_layout).init(
    //     &mut peripherals.RESETS,
    //     125_000_000u32.Hz(),
    //     16_000_000u32.Hz(),
    //     MODE_0,
    // );
    //
    // let dc_pin = pins.gpio16.into_push_pull_output();
    // let rst_pin = pins.gpio20.into_push_pull_output();
    //
    // let mut display = ST7735::new(spi, dc_pin, rst_pin, true, false, 160, 128);

    let rotary_clk = pins.gpio15.into_pull_up_input();
    let rotary_dt = pins.gpio14.into_pull_up_input();

    let mut rotary_encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

    loop {
        match rotary_encoder.update() {
            Direction::Clockwise => {
                info!("Rotate Clockwise")
            }
            Direction::Anticlockwise => {
                info!("Rotate Anticlockwise")
            }
            Direction::None => {}
        }
    }
}
