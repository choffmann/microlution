use byte_slice_cast::AsByteSlice;
use display::{DataFormat, DisplayError, ReadWriteDataCommand};
use embedded_hal::{digital::OutputPin, spi::SpiDevice};

pub mod client;
pub mod display;
pub mod input;
pub mod menu;
pub(crate) const BUFFER_SIZE: usize = 64;

pub struct SPIInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SPIInterface<SPI, DC> {
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

fn send_u8<SPI>(spi: &mut SPI, words: DataFormat<'_>) -> Result<(), DisplayError>
where
    SPI: SpiDevice,
{
    match words {
        DataFormat::U8(slice) => spi.write(slice).map_err(|_| DisplayError::BusWriteError),
        DataFormat::U16(slice) => spi
            .write(slice.as_byte_slice())
            .map_err(|_| DisplayError::BusWriteError),
        DataFormat::U16LE(slice) => {
            for v in slice.as_mut() {
                *v = v.to_le();
            }

            spi.write(slice.as_byte_slice())
                .map_err(|_| DisplayError::BusWriteError)
        }
        DataFormat::U16BE(slice) => {
            for v in slice.as_mut() {
                *v = v.to_be();
            }

            spi.write(slice.as_byte_slice())
                .map_err(|_| DisplayError::BusWriteError)
        }
        DataFormat::U8Iter(iter) => {
            let mut buf = [0; BUFFER_SIZE];
            let mut i = 0;
            for v in iter.into_iter() {
                buf[i] = v;
                i += 1;
                if i == buf.len() {
                    spi.write(&buf).map_err(|_| DisplayError::BusWriteError)?;
                    i = 0;
                }
            }

            if i > 0 {
                spi.write(&buf[..i])
                    .map_err(|_| DisplayError::BusWriteError)?;
            }
            Ok(())
        }
        DataFormat::U16LEIter(iter) => {
            let mut buf = [0; BUFFER_SIZE];
            let mut i = 0;
            for v in iter.map(u16::to_le) {
                buf[i] = v;
                i += 1;
                if i == buf.len() {
                    spi.write(buf.as_byte_slice())
                        .map_err(|_| DisplayError::BusWriteError)?;
                    i = 0;
                }
            }

            if i > 0 {
                spi.write(buf[..i].as_byte_slice())
                    .map_err(|_| DisplayError::BusWriteError)?;
            }

            Ok(())
        }

        DataFormat::U16BEIter(iter) => {
            let mut buf = [0; BUFFER_SIZE];
            let mut i = 0;
            let len = buf.len();
            for v in iter.map(u16::to_be) {
                buf[i] = v;
                i += 1;
                if i == len {
                    spi.write(buf.as_byte_slice())
                        .map_err(|_| DisplayError::BusWriteError)?;
                    i = 0;
                }
            }

            if i > 0 {
                spi.write(buf[..i].as_byte_slice())
                    .map_err(|_| DisplayError::BusWriteError)?;
            }

            Ok(())
        }
    }
}

impl<SPI, DC> ReadWriteDataCommand for SPIInterface<SPI, DC>
where
    SPI: SpiDevice,
    DC: OutputPin,
{
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), display::DisplayError> {
        self.dc
            .set_low()
            .map_err(|_| display::DisplayError::DCError)?;

        send_u8(&mut self.spi, cmd)
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), display::DisplayError> {
        self.dc
            .set_high()
            .map_err(|_| display::DisplayError::DCError)?;

        send_u8(&mut self.spi, buf)
    }

    fn read_data(
        &mut self,
        cmd: DataFormat<'_>,
        buf: &mut [u8],
    ) -> Result<(), display::DisplayError> {
        self.send_commands(cmd)?;
        self.dc
            .set_high()
            .map_err(|_| display::DisplayError::DCError)?;

        self.spi.read(buf).map_err(|_| DisplayError::BusReadError)
    }
}
