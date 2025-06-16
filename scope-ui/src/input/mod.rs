use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

pub enum InputEvent {
    Up,
    Down,
    Select,
    Quit,
}

pub trait MenuInput {
    fn poll(&mut self) -> Option<InputEvent>;
    fn update<D: DrawTarget<Color = BinaryColor>>(&mut self, _display: &mut D) {}
}
