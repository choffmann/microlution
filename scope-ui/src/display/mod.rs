pub mod graphics_core;
pub mod ili9341;

/// Trait for display interfaces that support both command and data transmission,
/// including optional read-back functionality for supported controllers.
///
/// This trait abstracts the low-level communication with a display controller.
/// It allows drivers to operate independently of the specific communication bus
/// (e.g., SPI, I²C, parallel) by accepting a flexible `DataFormat` abstraction.
pub trait ReadWriteDataCommand {
    /// Sends a display command.
    ///
    /// The command is typically a single byte or a sequence of command bytes
    /// without any associated data payload.
    ///
    /// # Arguments
    ///
    /// * `cmd` - One or more bytes representing the command(s) to send.
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError>;

    /// Sends data to the display following a command.
    ///
    /// This is typically used to transmit arguments or pixel data after sending a command.
    ///
    /// # Arguments
    ///
    /// * `buf` - Data payload to send.
    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError>;

    /// Reads data back from the display.
    ///
    /// This is used for reading controller status or pixel data if supported.
    ///
    /// # Arguments
    ///
    /// * `cmd` - Command to initiate the read (often a register read command).
    /// * `buf` - Mutable slice that will be filled with the received bytes.
    fn read_data(&mut self, cmd: DataFormat<'_>, buf: &mut [u8]) -> Result<(), DisplayError>;
}

/// Errors that may occur during display interface operations.
#[derive(Clone, Debug)]
pub enum DisplayError {
    /// The specified data format is not valid for the selected interface.
    InvalidFormatError,

    /// A bus write operation failed (e.g., SPI/I²C transmission failed).
    BusWriteError,

    /// A bus read operation failed (e.g., SPI/I²C reception failed).
    BusReadError,

    /// Failed to toggle the data/command (DC) pin.
    DCError,

    /// Failed to control the chip select (CS) pin.
    CSError,

    /// The specified `DataFormat` variant is not implemented by the interface.
    DataFormatNotImplemented,

    /// Failed to toggle the reset (RST) pin.
    RSError,

    /// Attempted to write to or read from an out-of-bounds pixel coordinate.
    OutOfBoundsError,
}

/// Describes the format of data to be sent or received over a display interface.
///
/// This abstraction enables flexible handling of data payloads, including slices and iterators
/// of various widths and endianness. Interfaces can choose which variants they support.
pub enum DataFormat<'a> {
    /// A slice of raw 8-bit unsigned integers.
    U8(&'a [u8]),

    /// Slice of unsigned 16bit values with the same endianness as the system, not recommended
    U16(&'a [u16]),

    /// A mutable slice of 16-bit values to be sent in **big-endian** byte order.
    U16BE(&'a mut [u16]),

    /// A mutable slice of 16-bit values to be sent in **little-endian** byte order.
    U16LE(&'a mut [u16]),

    /// A mutable iterator yielding 8-bit unsigned values.
    U8Iter(&'a mut dyn Iterator<Item = u8>),

    /// A mutable iterator yielding 16-bit values to be sent in **big-endian** format.
    U16BEIter(&'a mut dyn Iterator<Item = u16>),

    /// A mutable iterator yielding 16-bit values to be sent in **little-endian** format.
    U16LEIter(&'a mut dyn Iterator<Item = u16>),
}

/// Trait that defines the display's dimensions.
///
/// Used to configure width and height for different screen models.
/// Typical implementations are provided as unit structs (e.g. `DisplaySize240x320`).
pub trait DisplaySize {
    /// Width of the display in pixels
    const WIDTH: usize;

    /// Height of the display in pixels
    const HEIGHT: usize;
}
