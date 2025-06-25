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
            pin_state: [0u8; 3],
            last_click_time: Instant::now(),
            min_click_interval: Duration::from_millis(700),
        }
    }
}

const PIN_MASK: u8 = 0x03;
const PIN_EDGE: u8 = 0x02;

enum Direction {
    Clockwise,
    CounterClockwise,
    None,
}

impl From<u8> for Direction {
    fn from(s: u8) -> Self {
        match s {
            0b0001 | 0b0111 | 0b1000 | 0b1110 => Direction::Clockwise,
            0b0010 | 0b0100 | 0b1011 | 0b1101 => Direction::CounterClockwise,
            _ => Direction::None,
        }
    }
}

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

        let now = Instant::now();
        let mut event = None;

        if a == PIN_EDGE && b == 0x00 {
            debug!("rotary encoder down, a: 0x{:02x}, b: 0x{:02x}", a, b);
            event = Some(InputEvent::Down);
        } else if b == PIN_EDGE && a == 0x00 {
            debug!("rotary encoder up, a: 0x{:02x}, b: 0x{:02x}", a, b);
            event = Some(InputEvent::Up);
        }

        if self.pin_state[2] == 0x00
            && now.duration_since(self.last_click_time) >= self.min_click_interval
        {
            debug!("rotary encoder button click");
            self.last_click_time = now;
            self.pin_state[2] = 0;
            // event = Some(InputEvent::Select);
        }

        event

        // let mut s = self.state & 0b11;
        // if self.dt.is_low().unwrap() {
        //     s |= 0b100;
        // }
        // if self.clk.is_low().unwrap() {
        //     s |= 0b1000;
        // }
        //
        // // shift new to old
        // self.state = s >> 2;
        //
        // match s.into() {
        //     Direction::Clockwise => Some(InputEvent::Down),
        //     Direction::CounterClockwise => Some(InputEvent::Up),
        //     Direction::None => None,
        // }
    }
}
