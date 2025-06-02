pub mod ili9341;

/// Trait for displays that support both write and read operations.
pub trait ReadWriteDataCommand {
    /// Send command bytes to the display.
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError>;
    /// Send data bytes to the display.
    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError>;
    /// Read data from the display into `buf`, returns the filled slice.
    fn read_data(&mut self, cmd: DataFormat<'_>, buf: &mut [u8]) -> Result<(), DisplayError>;
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

/// Data format wrapper
pub enum DataFormat<'a> {
    /// Slice of unsigned bytes
    U8(&'a [u8]),
    /// Slice of unsigned 16bit values with the same endianness as the system, not recommended
    U16(&'a [u16]),
    /// Slice of unsigned 16bit values to be sent in big endian byte order
    U16BE(&'a mut [u16]),
    /// Slice of unsigned 16bit values to be sent in little endian byte order
    U16LE(&'a mut [u16]),
    /// Iterator over unsigned bytes
    U8Iter(&'a mut dyn Iterator<Item = u8>),
    /// Iterator over unsigned 16bit values to be sent in big endian byte order
    U16BEIter(&'a mut dyn Iterator<Item = u16>),
    /// Iterator over unsigned 16bit values to be sent in little endian byte order
    U16LEIter(&'a mut dyn Iterator<Item = u16>),
}
