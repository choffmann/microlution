use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use super::DisplayError;
use super::ReadWriteDataCommand;

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
    landscape: bool,
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
            landscape: false,
        };

        // Hardware reset by holding reset low for at least 10us
        ili9341.reset.set_low().map_err(|_| DisplayError::RSError)?;
        let _ = delay.delay_ms(1);

        // Set high for normal operation
        ili9341
            .reset
            .set_high()
            .map_err(|_| DisplayError::RSError)?;

        // Wait 5ms after reset before sending commands
        let _ = delay.delay_ms(5);

        // Do software reset
        ili9341.command(Command::SoftwareReset, None)?;

        // Wait 120ms before sending Sleep Out
        let _ = delay.delay_ms(120);

        // Set display to landscape mode
        ili9341.command(Command::MemoryAccessControl, Some(&[0x28]))?;

        // Set pixel format to 16 bits per pixel
        ili9341.command(Command::PixelFormatSet, Some(&[0x55]))?;

        ili9341.sleep_mode(ModeState::Off)?;

        // Wait 5ms after Sleep Out before sending commands
        let _ = delay.delay_ms(5);

        ili9341.display_mode(ModeState::On)?;

        Ok(ili9341)
    }

    fn command(&mut self, cmd: Command, args: Option<&[u8]>) -> Result {
        self.interface.send_commands(&[cmd as u8])?;
        if let Some(data) = args {
            return self.interface.send_data(data);
        }
        Ok(())
    }

    fn read(&mut self, cmd: Command, buf: &mut [u8]) -> Result {
        self.interface.read_data(&[cmd as u8], buf)
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

    pub fn status(&mut self) -> Result<[u8;5]> {
        let mut buf = [0u8; 5];
        self.read(Command::StatusInfo, &mut buf)?;
        Ok(buf)
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
    VerticalScrollDefine = 0x33,
    VerticalScrollAddr = 0x37,
    IdleModeOff = 0x38,
    IdleModeOn = 0x39,
    SetBrightness = 0x51,
    ContentAdaptiveBrightness = 0x55,
    NormalModeFrameRate = 0xb1,
    IdleModeFrameRate = 0xb2,
}
