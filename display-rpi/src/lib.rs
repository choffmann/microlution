use display::{DisplayError, ReadWriteDataCommand};
use embedded_hal::{digital::OutputPin, spi::SpiDevice};

pub mod display;
pub mod ui;

pub struct SPIInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SPIInterface<SPI, DC> {
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC> ReadWriteDataCommand for SPIInterface<SPI, DC>
where
    SPI: SpiDevice,
    DC: OutputPin,
{
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), display::DisplayError> {
        self.dc
            .set_low()
            .map_err(|_| display::DisplayError::DCError)?;

        self.spi
            .write(cmd)
            .map_err(|_| display::DisplayError::BusWriteError)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), display::DisplayError> {
        self.dc
            .set_high()
            .map_err(|_| display::DisplayError::DCError)?;

        self.spi
            .write(buf)
            .map_err(|_| display::DisplayError::BusWriteError)
    }

    fn read_data(&mut self, cmd: &[u8], buf: &mut [u8]) -> Result<(), display::DisplayError> {
        self.send_commands(cmd)?;
        self.dc
            .set_high()
            .map_err(|_| display::DisplayError::DCError)?;

        self.spi
            .read(buf)
            .map_err(|_| DisplayError::BusReadError)
    }
}
