use display_interface::DataFormat;
pub use display_interface::DisplayError;
use display_interface::WriteOnlyDataCommand;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use super::Flushable;

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

/// Generic display size of 320x480 pixels
pub struct DisplaySize320x480;

impl DisplaySize for DisplaySize320x480 {
    const WIDTH: usize = 320;
    const HEIGHT: usize = 480;
}

/// For quite a few boards (ESP32-S2-Kaluga-1, M5Stack, M5Core2 and others),
/// the ILI9341 initialization command arguments are slightly different
///
/// This trait provides the flexibility for users to define their own
/// initialization command arguments suitable for the particular board they are using
pub trait Mode {
    fn mode(&self) -> u8;

    fn is_landscape(&self) -> bool;
}

/// The default implementation of the Mode trait from above
/// Should work for most (but not all) boards
pub enum Orientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
}

impl Mode for Orientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0x40 | 0x08,
            Self::Landscape => 0x20 | 0x08,
            Self::PortraitFlipped => 0x80 | 0x08,
            Self::LandscapeFlipped => 0x40 | 0x80 | 0x20 | 0x08,
        }
    }

    fn is_landscape(&self) -> bool {
        match self {
            Self::Landscape | Self::LandscapeFlipped => true,
            Self::Portrait | Self::PortraitFlipped => false,
        }
    }
}

/// Specify state of specific mode of operation
pub enum ModeState {
    On,
    Off,
}

#[derive(Debug, Default, Clone, Copy)]
struct Pixel {
    x: u16,
    y: u16,
    color: u8,
}

impl Pixel {
    fn new(x: u16, y: u16, color: u8) -> Self {
        Self { x, y, color }
    }
}

const DISPLAY_WIDTH: usize = 320;
const DISPLAY_HEIGHT: usize = 240;

struct PixelBuff([Pixel; DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize]);

impl Default for PixelBuff {
    fn default() -> Self {
        let mut buff = Vec::with_capacity((DISPLAY_WIDTH * DISPLAY_HEIGHT).into());
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                buff.push(Pixel {
                    x: x as u16,
                    y: y as u16,
                    color: 0,
                });
            }
        }
        let buff = buff.try_into().unwrap_or_else(|v: Vec<Pixel>| {
            panic!(
                "Expected a Vec of length {} but it was {}",
                DISPLAY_WIDTH * DISPLAY_HEIGHT,
                v.len()
            )
        });
        Self(buff)
    }
}

/// There are two method for drawing to the screen:
/// [Ili9341::draw_raw_iter] and [Ili9341::draw_raw_slice]
///
/// In both cases the expected pixel format is rgb565.
///
/// The hardware makes it efficient to draw rectangles on the screen.
///
/// What happens is the following:
///
/// - A drawing window is prepared (with the 2 opposite corner coordinates)
/// - The starting point for drawint is the top left corner of this window
/// - Every pair of bytes received is intepreted as a pixel value in rgb565
/// - As soon as a pixel is received, an internal counter is incremented,
///   and the next word will fill the next pixel (the adjacent on the right, or
///   the first of the next row if the row ended)
pub struct Ili9341<IFACE, RESET> {
    interface: IFACE,
    reset: RESET,
    width: usize,
    height: usize,
    landscape: bool,
    drawn_buffer: PixelBuff,
    buffer: PixelBuff,
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
    RESET: OutputPin,
{
    pub fn new<DELAY, SIZE, MODE>(
        interface: IFACE,
        reset: RESET,
        delay: &mut DELAY,
        mode: MODE,
        _display_size: SIZE,
    ) -> Result<Self>
    where
        DELAY: DelayNs,
        SIZE: DisplaySize,
        MODE: Mode,
    {
        let mut ili9341 = Ili9341 {
            interface,
            reset,
            width: SIZE::WIDTH,
            height: SIZE::HEIGHT,
            landscape: false,
            drawn_buffer: PixelBuff::default(),
            buffer: PixelBuff::default(),
        };

        // Do hardware reset by holding reset low for at least 10us
        ili9341.reset.set_low().map_err(|_| DisplayError::RSError)?;
        let _ = delay.delay_ms(1);
        // Set high for normal operation
        ili9341
            .reset
            .set_high()
            .map_err(|_| DisplayError::RSError)?;

        // Wait 5ms after reset before sending commands
        // and 120ms before sending Sleep Out
        let _ = delay.delay_ms(5);

        // Do software reset
        ili9341.command(Command::SoftwareReset, &[])?;

        // Wait 5ms after reset before sending commands
        // and 120ms before sending Sleep Out
        let _ = delay.delay_ms(120);

        ili9341.set_orientation(mode)?;

        // Set pixel format to 16 bits per pixel
        ili9341.command(Command::PixelFormatSet, &[0x55])?;

        ili9341.sleep_mode(ModeState::Off)?;

        // Wait 5ms after Sleep Out before sending commands
        let _ = delay.delay_ms(5);

        ili9341.display_mode(ModeState::On)?;

        Ok(ili9341)
    }
}

impl<IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
{
    fn command(&mut self, cmd: Command, args: &[u8]) -> Result {
        self.interface.send_commands(DataFormat::U8(&[cmd as u8]))?;
        self.interface.send_data(DataFormat::U8(args))
    }

    fn write_iter<I: IntoIterator<Item = u16>>(&mut self, data: I) -> Result {
        self.command(Command::MemoryWrite, &[])?;
        use DataFormat::U16BEIter;
        self.interface.send_data(U16BEIter(&mut data.into_iter()))
    }

    fn write_slice(&mut self, data: &mut [u16]) -> Result {
        self.command(Command::MemoryWrite, &[])?;
        self.interface.send_data(DataFormat::U16BE(data))
    }

    fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result {
        self.command(
            Command::ColumnAddressSet,
            &[
                (x0 >> 8) as u8,
                (x0 & 0xff) as u8,
                (x1 >> 8) as u8,
                (x1 & 0xff) as u8,
            ],
        )?;
        self.command(
            Command::PageAddressSet,
            &[
                (y0 >> 8) as u8,
                (y0 & 0xff) as u8,
                (y1 >> 8) as u8,
                (y1 & 0xff) as u8,
            ],
        )
    }

    /// Configures the screen for hardware-accelerated vertical scrolling.
    pub fn configure_vertical_scroll(
        &mut self,
        fixed_top_lines: u16,
        fixed_bottom_lines: u16,
    ) -> Result<Scroller> {
        let height = if self.landscape {
            self.width
        } else {
            self.height
        } as u16;
        let scroll_lines = height as u16 - fixed_top_lines - fixed_bottom_lines;

        self.command(
            Command::VerticalScrollDefine,
            &[
                (fixed_top_lines >> 8) as u8,
                (fixed_top_lines & 0xff) as u8,
                (scroll_lines >> 8) as u8,
                (scroll_lines & 0xff) as u8,
                (fixed_bottom_lines >> 8) as u8,
                (fixed_bottom_lines & 0xff) as u8,
            ],
        )?;

        Ok(Scroller::new(fixed_top_lines, fixed_bottom_lines, height))
    }

    pub fn scroll_vertically(&mut self, scroller: &mut Scroller, num_lines: u16) -> Result {
        scroller.top_offset += num_lines;
        if scroller.top_offset > (scroller.height - scroller.fixed_bottom_lines) {
            scroller.top_offset = scroller.fixed_top_lines
                + (scroller.top_offset + scroller.fixed_bottom_lines - scroller.height)
        }

        self.command(
            Command::VerticalScrollAddr,
            &[
                (scroller.top_offset >> 8) as u8,
                (scroller.top_offset & 0xff) as u8,
            ],
        )
    }

    /// Draw a rectangle on the screen, represented by top-left corner (x0, y0)
    /// and bottom-right corner (x1, y1).
    ///
    /// The border is included.
    ///
    /// This method accepts an iterator of rgb565 pixel values.
    ///
    /// The iterator is useful to avoid wasting memory by holding a buffer for
    /// the whole screen when it is not necessary.
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

    /// Draw a rectangle on the screen, represented by top-left corner (x0, y0)
    /// and bottom-right corner (x1, y1).
    ///
    /// The border is included.
    ///
    /// This method accepts a raw buffer of words that will be copied to the screen
    /// video memory.
    ///
    /// The expected format is rgb565.
    pub fn draw_raw_slice(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        data: &mut [u16],
    ) -> Result {
        self.set_window(x0, y0, x1, y1)?;
        self.write_slice(data)
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, value: u8) {
        let idx = self.width * y as usize + x as usize;
        self.buffer.0[idx] = Pixel { x, y, color: value };
    }

    /// Change the orientation of the screen
    pub fn set_orientation<MODE>(&mut self, mode: MODE) -> Result
    where
        MODE: Mode,
    {
        self.command(Command::MemoryAccessControl, &[mode.mode()])?;

        if self.landscape ^ mode.is_landscape() {
            core::mem::swap(&mut self.height, &mut self.width);
        }
        self.landscape = mode.is_landscape();
        Ok(())
    }

    /// Fill entire screen with specfied color u16 value
    pub fn clear_screen(&mut self, color: u16) -> Result {
        let color = core::iter::repeat(color).take(self.width * self.height);
        self.draw_raw_iter(0, 0, self.width as u16, self.height as u16, color)
    }

    /// Control the screen sleep mode:
    pub fn sleep_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::SleepModeOn, &[]),
            ModeState::Off => self.command(Command::SleepModeOff, &[]),
        }
    }

    /// Control the screen display mode
    pub fn display_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::DisplayOn, &[]),
            ModeState::Off => self.command(Command::DisplayOff, &[]),
        }
    }

    /// Invert the pixel color on screen
    pub fn invert_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::InvertOn, &[]),
            ModeState::Off => self.command(Command::InvertOff, &[]),
        }
    }

    /// Idle mode reduces the number of colors to 8
    pub fn idle_mode(&mut self, mode: ModeState) -> Result {
        match mode {
            ModeState::On => self.command(Command::IdleModeOn, &[]),
            ModeState::Off => self.command(Command::IdleModeOff, &[]),
        }
    }

    /// Set display brightness to the value between 0 and 255
    pub fn brightness(&mut self, brightness: u8) -> Result {
        self.command(Command::SetBrightness, &[brightness])
    }

    /// Set adaptive brightness value equal to [AdaptiveBrightness]
    pub fn content_adaptive_brightness(&mut self, value: AdaptiveBrightness) -> Result {
        self.command(Command::ContentAdaptiveBrightness, &[value as _])
    }

    /// Configure [FrameRateClockDivision] and [FrameRate] in normal mode
    pub fn normal_mode_frame_rate(
        &mut self,
        clk_div: FrameRateClockDivision,
        frame_rate: FrameRate,
    ) -> Result {
        self.command(
            Command::NormalModeFrameRate,
            &[clk_div as _, frame_rate as _],
        )
    }

    /// Configure [FrameRateClockDivision] and [FrameRate] in idle mode
    pub fn idle_mode_frame_rate(
        &mut self,
        clk_div: FrameRateClockDivision,
        frame_rate: FrameRate,
    ) -> Result {
        self.command(Command::IdleModeFrameRate, &[clk_div as _, frame_rate as _])
    }
}

impl<IFACE, RESET> Flushable for Ili9341<IFACE, RESET>
where
    IFACE: WriteOnlyDataCommand,
{
    fn flush(&mut self) -> std::result::Result<(), DisplayError> {
        let default_min = Pixel {
            x: self.width as u16,
            y: self.height as u16,
            color: 0,
        };
        let default_max = Pixel {
            x: 0,
            y: 0,
            color: 0,
        };
        let (min, max) = self.drawn_buffer.0.iter().zip(self.buffer.0.iter()).fold(
            (default_min, default_max),
            |acc, (prev, new)| {
                if prev.color ^ new.color == 0 {
                    return acc;
                } else {
                    let (min, max) = acc;
                    return (
                        Pixel::new(new.x.min(min.x), new.y.min(min.y), new.color),
                        Pixel::new(new.x.max(max.x), new.y.max(max.y), new.color),
                    );
                }
            },
        );

        let mut data = Vec::with_capacity(
            ((max.x.wrapping_sub(min.x).wrapping_add(1))
                * (max.y.wrapping_sub(min.y).wrapping_add(1)))
            .into(),
        );
        for y0 in min.y..=max.y {
            for x0 in min.x..=max.x {
                let idx = self.width * y0 as usize + x0 as usize;
                let color = if self.buffer.0[idx].color == 0 {
                    0x0000
                } else {
                    0xFFFF
                };
                data.push(color);
            }
        }
        // dbg!(min, max, &data);
        self.drawn_buffer.0 = self.buffer.0.clone();
        self.buffer = PixelBuff::default();
        self.draw_raw_slice(min.x, min.y, max.x, max.y, &mut data)
            .map_err(|e| {
                match e {
                    DisplayError::BusWriteError => {
                        println!("Failed to write to display {:?}", e);
                        dbg!(min, max);
                        println!("0x{:02X?}", data);
                        print(min, max, &data);
                        println!();
                    }
                    _ => {}
                }
                return e;
            })
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

/// Scroller must be provided in order to scroll the screen. It can only be obtained
/// by configuring the screen for scrolling.
pub struct Scroller {
    top_offset: u16,
    fixed_bottom_lines: u16,
    fixed_top_lines: u16,
    height: u16,
}

impl Scroller {
    fn new(fixed_top_lines: u16, fixed_bottom_lines: u16, height: u16) -> Scroller {
        Scroller {
            top_offset: fixed_top_lines,
            fixed_top_lines,
            fixed_bottom_lines,
            height,
        }
    }
}

/// Available Adaptive Brightness values
pub enum AdaptiveBrightness {
    Off = 0x00,
    UserInterfaceImage = 0x01,
    StillPicture = 0x02,
    MovingImage = 0x03,
}

/// Available frame rate in Hz
pub enum FrameRate {
    FrameRate119 = 0x10,
    FrameRate112 = 0x11,
    FrameRate106 = 0x12,
    FrameRate100 = 0x13,
    FrameRate95 = 0x14,
    FrameRate90 = 0x15,
    FrameRate86 = 0x16,
    FrameRate83 = 0x17,
    FrameRate79 = 0x18,
    FrameRate76 = 0x19,
    FrameRate73 = 0x1a,
    FrameRate70 = 0x1b,
    FrameRate68 = 0x1c,
    FrameRate65 = 0x1d,
    FrameRate63 = 0x1e,
    FrameRate61 = 0x1f,
}

/// Frame rate clock division
pub enum FrameRateClockDivision {
    Fosc = 0x00,
    FoscDiv2 = 0x01,
    FoscDiv4 = 0x02,
    FoscDiv8 = 0x03,
}

#[derive(Clone, Copy)]
enum Command {
    SoftwareReset = 0x01,
    MemoryAccessControl = 0x36,
    PixelFormatSet = 0x3a,
    SleepModeOn = 0x10,
    SleepModeOff = 0x11,
    InvertOff = 0x20,
    InvertOn = 0x21,
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

fn print(min: Pixel, max: Pixel, to_draw: &[u16]) {
    for y in 0..=(max.y - min.y) {
        for x in 0..=(max.x - min.x) {
            let idx = (max.x - min.x + 1) * y + x;
            if to_draw[idx as usize] == 0x00 {
                print!(" ")
            } else {
                print!("█")
            }
        }
        println!()
    }
}
