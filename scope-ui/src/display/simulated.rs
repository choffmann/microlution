use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};
use embedded_graphics_simulator::SimulatorDisplay;

use super::DisplaySize;

#[derive(Debug)]
pub struct SimulatedDisplay;

impl SimulatedDisplay {
    pub fn build<SIZE>(_display_size: SIZE) -> SimulatorDisplay<Rgb565>
    where
        SIZE: DisplaySize,
    {
        let width: u32 = SIZE::WIDTH as u32;
        let height: u32 = SIZE::HEIGHT as u32;
        SimulatorDisplay::new(Size::new(width, height))
    }
}
