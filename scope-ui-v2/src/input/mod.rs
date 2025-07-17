pub mod rotary_encoder;

#[derive(Debug)]
pub enum InputEvent {
    Up,
    Down,
    Select,
    Quit,
}

pub trait MenuInput {
    fn poll(&mut self) -> Option<InputEvent>;
}
