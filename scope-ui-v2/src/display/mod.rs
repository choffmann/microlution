use ::ili9341::DisplayError;

pub mod graphics_core;
pub mod ili9341;

pub trait Flushable {
    fn flush(&mut self) -> Result<(), DisplayError>;
}

