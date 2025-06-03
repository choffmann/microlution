use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use super::DisplayError;
use super::ReadWriteDataCommand;
use super::DataFormat;

type Result<T = (), E = DisplayError> = core::result::Result<T, E>;

/// Trait that defines display size information
pub trait DisplaySize {
    /// Width in pixels
    const WIDTH: usize;
    /// Height in pixels
    const HEIGHT: usize;
}

/// Generic display size of 240x320 pixels
pub struct DisplaySize240x320;

impl DisplaySize for DisplaySize240x320 {
    const WIDTH: usize = 240;
    const HEIGHT: usize = 320;
}

/// Specify state of specific mode of operation
pub enum ModeState {
    On,
    Off,
}

pub struct Ili9341<IFACE, RESET> {
    interface: IFACE,
    reset: RESET,
    width: usize,
    height: usize,
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: ReadWriteDataCommand,
    RESET: OutputPin,
{
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
        ili9341.command(Command::MemoryAccessControl, Some(&[0x40 | 0x08]))?;

        // Set pixel format to 16 bits per pixel
        ili9341.command(Command::PixelFormatSet, Some(&[0x55]))?;

        ili9341.sleep_mode(ModeState::Off)?;

        // Wait 5ms after Sleep Out before sending commands
        delay.delay_ms(5);

        ili9341.display_mode(ModeState::On)?;

        Ok(ili9341)
    }

    fn command(&mut self, cmd: Command, args: Option<&[u8]>) -> Result {
        self.interface.send_commands(DataFormat::U8(&[cmd as u8]))?;
        if let Some(data) = args {
            return self.interface.send_data(DataFormat::U8(data))
        }
        Ok(())
    }

    fn read(&mut self, cmd: Command, buf: &mut [u8]) -> Result {
        self.interface.read_data(DataFormat::U8(&[cmd as u8]), buf)
    }

    fn write_iter<I: IntoIterator<Item = u16>>(&mut self, data: I) -> Result {
        self.command(Command::MemoryWrite, None)?;
        self.interface.send_data(DataFormat::U16BEIter(&mut data.into_iter()))
    }

    fn write_slice(&mut self, data: &[u16]) -> Result {
        self.command(Command::MemoryWrite, None)?;
        self.interface.send_data(DataFormat::U16(data))
    }

    fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result {
        self.command(
            Command::ColumnAddressSet,
            Some(&[
                (x0 >> 8) as u8,
                (x0 & 0xff) as u8,
                (x1 >> 8) as u8,
                (x1 & 0xff) as u8,
            ]),
        )?;

        self.command(
            Command::PageAddressSet,
            Some(&[
                (y0 >> 8) as u8,
                (y0 & 0xff) as u8,
                (y1 >> 8) as u8,
                (y1 & 0xff) as u8,
            ]),
        )
    }

    pub fn draw_raw_iter<I: IntoIterator<Item = u16>>(
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

    pub fn draw_raw_slice(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, data: &[u16]) -> Result {
        self.set_window(x0, y0, x1, y1)?;
        self.write_slice(data)
    }

    /// Control the screen sleep mode:
    pub fn sleep_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::SleepModeOn, None),
            ModeState::Off => self.command(Command::SleepModeOff, None),
        }
    }

    /// Control the screen display mode
    pub fn display_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::DisplayOn, None),
            ModeState::Off => self.command(Command::DisplayOff, None),
        }
    }

    /// Set display brightness to the value between 0 and 255
    pub fn brightness(&mut self, brightness: u8) -> Result {
        self.command(Command::SetBrightness, Some(&[brightness]))
    }

    pub fn status(&mut self) -> Result<[u8; 5]> {
        let mut buf = [0u8; 5];
        self.read(Command::StatusInfo, &mut buf)?;
        Ok(buf)
    }

    /// Fill entire screen with specfied color u16 value
    pub fn clear_screen(&mut self, color: u16) -> Result {
        let color = core::iter::repeat_n(color, self.width * self.height);
        self.draw_raw_iter(0, 0, self.width as u16, self.height as u16, color)
    }
}

impl<IFACE, RESET> Ili9341<IFACE, RESET> {
    /// Get the current screen width. It can change based on the current orientation
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the current screen heighth. It can change based on the current orientation
    pub fn height(&self) -> usize {
        self.height
    }
}

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
        const WIDTH: u16 = 2;
        const HEIGHT: u16 = 2;
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

        display.clear_screen(0x1234).unwrap();

        let binding = iface.borrow();
        let data = binding.data.borrow();

        let expected_bytes = vec![0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34];
        assert_eq!(&data[data.len() - 8..], &expected_bytes[..]);
    }
}
