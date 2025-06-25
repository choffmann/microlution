use embedded_hal::digital::InputPin;
use log::debug;

use super::{InputEvent, MenuInput};

pub struct RotaryEncoder<DT, CLK, SW> {
    dt: DT,
    clk: CLK,
    sw: SW,
    btn_state: u16,
    rotary_state: [u16; 2],
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
            btn_state: 0,
            rotary_state: [0u16; 2],
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
        const ROT_ENC_TABLE: [u8; 16] = [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0];

        let dt_value = self.dt.is_high().unwrap_or_default();
        let clk_value = self.clk.is_high().unwrap_or_default();
        let sw_value = self.sw.is_low().unwrap_or_default();

        self.rotary_state[0] <<= 2;
        if dt_value {
            self.rotary_state[0] |= 0x02
        }
        if clk_value {
            self.rotary_state[0] |= 0x01
        }
        self.rotary_state[0] &= 0x0f;

        self.btn_state = (self.btn_state << 1) | sw_value as u16 | 0xfe00;
        if self.btn_state == 0xff00 {
            debug!("rotary encoder button click");
            return Some(InputEvent::Select);
        }

        if self.rotary_state[0] == 0x0b {
            debug!("eleven 0x{:2X}", self.rotary_state[0])
        }

        if self.rotary_state[0] == 0x07 {
            debug!("seven 0x{:2X}", self.rotary_state[0])
        }

        if ROT_ENC_TABLE[self.rotary_state[0] as usize] != 0 {
            self.rotary_state[1] <<= 4;
            self.rotary_state[1] |= self.rotary_state[0];

            if (self.rotary_state[1] & 0xff) == 0x2b {
                debug!("rotary encoder up");
                return Some(InputEvent::Up);
            }

            if (self.rotary_state[1] & 0xff) == 0x17 {
                debug!("rotary encoder down");
                return Some(InputEvent::Down);
            }
        }

        None
    }
}
