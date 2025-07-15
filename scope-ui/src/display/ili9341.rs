use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use crate::display::Flushable;

use super::DataFormat;
use super::DisplayError;
use super::DisplaySize;
use super::ReadWriteDataCommand;

type Result<T = (), E = DisplayError> = core::result::Result<T, E>;

/// Predefined display size of 240x320 pixels.
pub struct DisplaySize320x240;

impl DisplaySize for DisplaySize320x240 {
    const WIDTH: usize = 320;
    const HEIGHT: usize = 240;
}

/// Enum indicating the on/off state of display modes like sleep or display power.
pub enum ModeState {
    /// Mode is active.
    On,
    /// Mode is inactive.
    Off,
}

/// Driver for ILI9341-based TFT displays.
///
/// This struct abstracts the SPI interface and reset pin, and provides high-level methods
/// to control the display.
///
/// # Type Parameters
/// - `IFACE`: Interface that implements the [`ReadWriteDataCommand`] trait.
/// - `RESET`: Digital output pin used to reset the display.
pub struct Ili9341<IFACE, RESET> {
    interface: IFACE,
    framebuffer: [[u16; 320]; 240], // WITDH x HEIGHT
    reset: RESET,
    width: usize,
    height: usize,
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: ReadWriteDataCommand,
    RESET: OutputPin,
{
    /// Initializes the ILI9341 display with given interface and reset pin.
    ///
    /// Performs a hardware and software reset, configures display format and exits sleep mode.
    ///
    /// # Arguments
    /// * `interface` – Interface implementing `ReadWriteDataCommand` (e.g. SPI + DC pin).
    /// * `reset` – Output pin connected to the RESET line of the display.
    /// * `delay` – Delay provider (must support millisecond delays).
    /// * `_display_size` – Type that implements [`DisplaySize`] to configure resolution.
    pub fn new<DELAY, SIZE>(
        interface: IFACE,
        reset: RESET,
        delay: &mut DELAY,
        _display_size: SIZE,
    ) -> Result<Self>
    where
        DELAY: DelayNs,
        SIZE: DisplaySize,
    {
        let mut ili9341 = Ili9341 {
            interface,
            reset,
            framebuffer: [[0u16; 320]; 240],
            width: SIZE::WIDTH,
            height: SIZE::HEIGHT,
        };

        // Hardware reset by holding reset low for at least 10us
        ili9341.reset.set_low().map_err(|_| DisplayError::RSError)?;
        delay.delay_ms(1);

        // Set high for normal operation
        ili9341
            .reset
            .set_high()
            .map_err(|_| DisplayError::RSError)?;

        // Wait 5ms after reset before sending commands
        delay.delay_ms(5);

        // Do software reset
        ili9341.command(Command::SoftwareReset, None)?;

        // Wait 120ms before sending Sleep Out
        delay.delay_ms(120);

        // Set display to landscape mode
        // 0x40 | 0x08 => Portrait
        // 0x20 | 0x08 => Landscape
        // 0x80 | 0x08 => Portrait flipped
        // 0x40 | 0x80 | 0x20 | 0x08 => Landscape flipped
        ili9341.command(
            Command::MemoryAccessControl,
            Some(&[0x40 | 0x80 | 0x20 | 0x08]),
        )?;

        // Set pixel format to 16 bits per pixel
        ili9341.command(Command::PixelFormatSet, Some(&[0x55]))?;

        ili9341.sleep_mode(ModeState::Off)?;

        // Wait 5ms after Sleep Out before sending commands
        delay.delay_ms(5);

        ili9341.display_mode(ModeState::On)?;

        Ok(ili9341)
    }

    /// Sends a command followed by optional arguments to the display.
    ///
    /// This is a low-level function and typically used internally.
    fn command(&mut self, cmd: Command, args: Option<&[u8]>) -> Result {
        self.interface
            .send_commands(DataFormat::U8(&[cmd.into()]))?;
        if let Some(data) = args {
            return self.interface.send_data(DataFormat::U8(data));
        }
        Ok(())
    }

    /// Reads data from the display after sending a command.
    fn read(&mut self, cmd: Command, buf: &mut [u8]) -> Result {
        self.interface.read_data(DataFormat::U8(&[cmd.into()]), buf)
    }

    /// Draws pixel data into the display memory using an iterator over 16-bit RGB565 values.
    ///
    /// Must be preceded by a `MemoryWrite` command.
    fn write_iter<I: IntoIterator<Item = u16>>(&mut self, data: I) -> Result {
        self.command(Command::MemoryWrite, None)?;
        self.interface
            .send_data(DataFormat::U16BEIter(&mut data.into_iter()))
    }

    /// Draws pixel data from a slice of 16-bit RGB565 values.
    fn write_slice(&mut self, data: &[u16]) -> Result {
        self.command(Command::MemoryWrite, None)?;
        self.interface.send_data(DataFormat::U16(data))
    }

    /// Sets the drawing area (window) on the display, defined by top-left and bottom-right coordinates.
    ///
    /// Coordinates are inclusive and must be within display bounds.
    fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result {
        self.command(Command::ColumnAddressSet, Some(&pack_coords(x0, x1)))?;
        self.command(Command::PageAddressSet, Some(&pack_coords(y0, y1)))
    }

    /// Draws an area of pixels using an iterator over 16-bit RGB565 values.
    ///
    /// The target area is defined by `(x0, y0)` to `(x1, y1)`, inclusive.
    pub fn draw_pixels_iter<I: IntoIterator<Item = u16>>(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        data: I,
    ) -> Result {
        self.set_window(x0, y0, x1, y1)?;
        self.write_iter(data)
    }

    pub fn write_pixel(&mut self, x: u16, y: u16, data: u16) -> Result {
        // dbg!(x, y, data);
        let (x, y) = (x as usize, y as usize);
        if x <= self.width && y <= self.height() {
            self.framebuffer[y][x] = data;
            Ok(())
        } else {
            Err(DisplayError::OutOfBoundsError)
        }
    }

    /// Draws an area of pixels from a slice of 16-bit RGB565 values.
    ///
    /// The length of the slice must match the number of pixels in the target rectangle.
    pub fn draw_pixels_slice(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        data: &[u16],
    ) -> Result {
        self.set_window(x0, y0, x1, y1)?;
        self.write_slice(data)
    }

    /// Enables or disables sleep mode on the display.
    ///
    /// Use `ModeState::Off` to wake the display, and `ModeState::On` to enter sleep.
    pub fn sleep_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::SleepModeOn, None),
            ModeState::Off => self.command(Command::SleepModeOff, None),
        }
    }

    /// Enables or disables the display output (power on/off).
    pub fn display_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::DisplayOn, None),
            ModeState::Off => self.command(Command::DisplayOff, None),
        }
    }

    /// Sets the display brightness to a value between 0 and 255.
    pub fn brightness(&mut self, brightness: u8) -> Result {
        self.command(Command::SetBrightness, Some(&[brightness]))
    }

    /// Reads status information from the display.
    ///
    /// Returns 5 bytes of raw status data.
    pub fn status(&mut self) -> Result<[u8; 5]> {
        let mut buf = [0u8; 5];
        self.read(Command::StatusInfo, &mut buf)?;
        Ok(buf)
    }

    /// Fills the entire display with a single RGB565 color.
    pub fn clear_screen(&mut self, color: u16) {
        self.framebuffer = [[color; 320]; 240];
    }
}

impl<IFACE, RESET> Ili9341<IFACE, RESET> {
    /// Returns the current width of the display in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the current height of the display in pixels.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<IFACE, RESET> Flushable for Ili9341<IFACE, RESET>
where
    IFACE: ReadWriteDataCommand,
    RESET: OutputPin,
{
    fn flush(&mut self) -> std::result::Result<(), DisplayError> {
        // dbg!("flush", self.framebuffer);
        self.draw_pixels_iter(
            0,
            0,
            self.width as u16,
            self.height as u16,
            self.framebuffer.into_iter().flatten(),
        )
    }
}

fn pack_coords(start: u16, end: u16) -> [u8; 4] {
    [
        (start >> 8) as u8,
        (start & 0xff) as u8,
        (end >> 8) as u8,
        (end & 0xff) as u8,
    ]
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Command {
    SoftwareReset = 0x01,
    StatusInfo = 0x09,
    MemoryAccessControl = 0x36,
    PixelFormatSet = 0x3a,
    SleepModeOn = 0x10,
    SleepModeOff = 0x11,
    DisplayOff = 0x28,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2a,
    PageAddressSet = 0x2b,
    MemoryWrite = 0x2c,
    SetBrightness = 0x51,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> u8 {
        cmd as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;
    use embedded_hal::delay::DelayNs;
    use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin};
    use std::rc::Rc;

    /// Dummy display size for testing
    struct DummySize;

    impl DisplaySize for DummySize {
        const WIDTH: usize = 2;
        const HEIGHT: usize = 2;
    }

    /// Mock implementation of Delay
    struct DummyDelay;
    impl DelayNs for DummyDelay {
        fn delay_ns(&mut self, _ns: u32) {}
        fn delay_us(&mut self, _us: u32) {}
        fn delay_ms(&mut self, _ms: u32) {}
    }

    /// Simple mock reset pin
    #[derive(Default)]
    struct DummyPin {
        pub set_low_called: bool,
        pub set_high_called: bool,
    }

    #[derive(Debug)]
    struct DummyError;

    impl Error for DummyError {
        fn kind(&self) -> ErrorKind {
            return ErrorKind::Other;
        }
    }

    impl ErrorType for DummyPin {
        type Error = DummyError;
    }

    impl OutputPin for DummyPin {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.set_low_called = true;
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.set_high_called = true;
            Ok(())
        }
    }

    /// Records sent commands and data
    #[derive(Default)]
    struct DummyInterface {
        pub commands: RefCell<Vec<u8>>,
        pub data: RefCell<Vec<u8>>,
    }

    impl ReadWriteDataCommand for DummyInterface {
        fn send_commands(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
            if let DataFormat::U8(slice) = data {
                self.commands.borrow_mut().extend_from_slice(slice);
            }
            Ok(())
        }

        fn send_data(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
            match data {
                DataFormat::U8(slice) => self.data.borrow_mut().extend_from_slice(slice),
                DataFormat::U16(slice) => {
                    for val in slice {
                        self.data.borrow_mut().extend_from_slice(&val.to_be_bytes());
                    }
                }
                DataFormat::U16BEIter(iter) => {
                    for val in iter {
                        self.data.borrow_mut().extend_from_slice(&val.to_be_bytes());
                    }
                }
                _ => return Err(DisplayError::DataFormatNotImplemented),
            }
            Ok(())
        }

        fn read_data(&mut self, _cmd: DataFormat<'_>, buf: &mut [u8]) -> Result<(), DisplayError> {
            buf.fill(0xAB); // dummy value
            Ok(())
        }
    }

    impl ReadWriteDataCommand for Rc<RefCell<DummyInterface>> {
        fn send_commands(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
            self.borrow_mut().send_commands(data)
        }

        fn send_data(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
            self.borrow_mut().send_data(data)
        }

        fn read_data(
            &mut self,
            cmd: DataFormat<'_>,
            buffer: &mut [u8],
        ) -> Result<(), DisplayError> {
            self.borrow_mut().read_data(cmd, buffer)
        }
    }

    #[test]
    fn test_initialization() {
        let iface = DummyInterface::default();
        let reset = DummyPin::default();
        let mut delay = DummyDelay;

        let display = Ili9341::new(iface, reset, &mut delay, DummySize);

        assert!(display.is_ok());

        let display = display.unwrap();
        assert_eq!(display.width(), 2);
        assert_eq!(display.height(), 2);
    }

    #[test]
    fn test_clear_screen() {
        let iface = Rc::new(RefCell::new(DummyInterface::default()));
        let reset = DummyPin::default();
        let mut delay = DummyDelay;

        let mut display = Ili9341::new(iface.clone(), reset, &mut delay, DummySize).unwrap();

        display.clear_screen(0x1234);

        let binding = iface.borrow();
        let data = binding.data.borrow();

        let expected_bytes = vec![0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34];
        assert_eq!(&data[data.len() - 8..], &expected_bytes[..]);
    }
}
