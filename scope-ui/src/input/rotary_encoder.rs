use std::time::{Duration, Instant};

use embedded_hal::digital::InputPin;
use log::debug;

use super::{InputEvent, MenuInput};

pub struct RotaryEncoder<DT, CLK, SW> {
    dt: DT,
    clk: CLK,
    sw: SW,
    pin_state: [u8; 3],
    last_click_time: Instant,
    min_click_interval: Duration,
}

impl<DT, CLK, SW> RotaryEncoder<DT, CLK, SW>
where
    DT: InputPin,
    CLK: InputPin,
    SW: InputPin,
{
    pub fn new(dt: DT, clk: CLK, sw: SW) -> Self {
        Self {
            dt,
            clk,
            sw,
            pin_state: [0xFF; 3],
            last_click_time: Instant::now(),
            min_click_interval: Duration::from_millis(700),
        }
    }
}

const PIN_MASK: u8 = 0x03;
const PIN_EDGE: u8 = 0x02;
const DEBOUNCE_MASK: u8 = 0x0f;

impl<DT, CLK, SW> MenuInput for RotaryEncoder<DT, CLK, SW>
where
    DT: InputPin,
    CLK: InputPin,
    SW: InputPin,
{
    fn poll(&mut self) -> Option<InputEvent> {
        let dt_value = self.dt.is_high().unwrap_or_default();
        let clk_value = self.clk.is_high().unwrap_or_default();
        let sw_value = self.sw.is_low().unwrap_or_default();

        self.pin_state[0] = (self.pin_state[0] << 1) | dt_value as u8;
        self.pin_state[1] = (self.pin_state[1] << 1) | clk_value as u8;

        let sw_bit = if sw_value { 0 } else { 1 };
        self.pin_state[2] = (self.pin_state[2] << 1) | sw_bit;

        let a = self.pin_state[0] & PIN_MASK;
        let b = self.pin_state[1] & PIN_MASK;
        let sw = self.pin_state[2] & DEBOUNCE_MASK;

        let now = Instant::now();
        let mut event = None;

        if a == PIN_EDGE && b == 0x00 {
            debug!("rotary encoder down, a: 0x{:02x}, b: 0x{:02x}", a, b);
            event = Some(InputEvent::Down);
        } else if b == PIN_EDGE && a == 0x00 {
            debug!("rotary encoder up, a: 0x{:02x}, b: 0x{:02x}", a, b);
            event = Some(InputEvent::Up);
        }

        if sw == 0x00 && now.duration_since(self.last_click_time) >= self.min_click_interval {
            debug!("rotary encoder button click");
            self.last_click_time = now;
            event = Some(InputEvent::Select);
        }

        event
    }
}
