pub mod ili9341;

/// Trait for displays that support both write and read operations.
pub trait ReadWriteDataCommand {
    /// Send command bytes to the display.
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), DisplayError>;
    /// Send data bytes to the display.
    fn send_data(&mut self, buf: &[u8]) -> Result<(), DisplayError>;
    /// Read data from the display into `buf`, returns the filled slice.
    fn read_data(&mut self, cmd: &[u8], buf: &mut [u8]) -> Result<(), DisplayError>;
}

#[derive(Clone, Debug)]
pub enum DisplayError {
    /// Invalid data format selected for interface selected
    InvalidFormatError,
    /// Unable to write to bus
    BusWriteError,
    /// Unable to read to bus
    BusReadError,
    /// Unable to assert or de-assert data/command switching signal
    DCError,
    /// Unable to assert chip select signal
    CSError,
    /// The requested DataFormat is not implemented by this display interface implementation
    DataFormatNotImplemented,
    /// Unable to assert or de-assert reset signal
    RSError,
    /// Attempted to write to a non-existing pixel outside the display's bounds
    OutOfBoundsError,
}
