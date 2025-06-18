use std::time::{Duration, Instant};

use rppal::gpio::InputPin;

use super::{InputEvent, MenuInput};

pub struct RotaryEncoder {
    dt: InputPin,
    clk: InputPin,
    sw: InputPin,
    last_clk: bool,
    last_sw: bool,
    last_click_time: Instant,
    min_click_interval: Duration,
}

impl RotaryEncoder {
    pub fn new(dt: InputPin, clk: InputPin, sw: InputPin) -> Self {
        let last_clk = clk.is_high();
        let last_sw = sw.is_high();
        Self {
            dt,
            clk,
            sw,
            last_clk,
            last_sw,
            last_click_time: Instant::now(),
            min_click_interval: Duration::from_millis(100),
        }
    }
}

impl MenuInput for RotaryEncoder {
    fn poll(&mut self) -> Option<InputEvent> {
        let clk_now = self.clk.is_high();
        let dt_now = self.dt.is_high();

        let mut event = None;

        if clk_now != self.last_clk {
            self.last_clk = clk_now;
            if clk_now {
                event = Some(if dt_now {
                    InputEvent::Up
                } else {
                    InputEvent::Down
                });
            };
        };

        let btn_now = self.sw.is_low();
        if btn_now != self.last_sw {
            self.last_sw = btn_now;
            let now = Instant::now();
            if btn_now && now - self.last_click_time > self.min_click_interval {
                self.last_click_time = now;
                return Some(InputEvent::Select);
            }
        }

        event
    }
}
