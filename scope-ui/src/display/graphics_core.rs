use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::*,
};
use embedded_hal::digital::OutputPin;

use super::{ili9341::Ili9341, DisplayError, ReadWriteDataCommand};

impl<IFACE, RESET> OriginDimensions for Ili9341<IFACE, RESET> {
    fn size(&self) -> Size {
        Size::new(self.width() as u32, self.height() as u32)
    }
}

impl<IFACE, RESET> DrawTarget for Ili9341<IFACE, RESET>
where
    IFACE: ReadWriteDataCommand,
    RESET: OutputPin,
{
    type Color = Rgb565;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if self.bounding_box().contains(point) {
                let x = point.x as u16;
                let y = point.y as u16;
                let color = RawU16::from(color).into_inner();
                self.write_pixel(x, y, color)?;
            }
        }
        Ok(())
    }

    // fn fill_contiguous<I>(
    //     &mut self,
    //     area: &embedded_graphics::primitives::Rectangle,
    //     colors: I,
    // ) -> Result<(), Self::Error>
    // where
    //     I: IntoIterator<Item = Self::Color>,
    // {
    //     let drawable_area = area.intersection(&self.bounding_box());
    //
    //     if let Some(drawable_bottom_right) = drawable_area.bottom_right() {
    //         let x0 = drawable_area.top_left.x as u16;
    //         let y0 = drawable_area.top_left.y as u16;
    //         let x1 = drawable_bottom_right.x as u16;
    //         let y1 = drawable_bottom_right.y as u16;
    //
    //         if area == &drawable_area {
    //             // All pixels are on screen
    //             self.draw_pixels_iter(
    //                 x0,
    //                 y0,
    //                 x1,
    //                 y1,
    //                 area.points()
    //                     .zip(colors)
    //                     .map(|(_, color)| RawU16::from(color).into_inner()),
    //             )
    //         } else {
    //             // Some Pixel are on screen
    //             self.draw_pixels_iter(
    //                 x0,
    //                 y0,
    //                 x1,
    //                 y1,
    //                 area.points()
    //                     .zip(colors)
    //                     .filter(|(point, _)| drawable_area.contains(*point))
    //                     .map(|(_, color)| RawU16::from(color).into_inner()),
    //             )
    //         }
    //     } else {
    //         Ok(())
    //     }
    // }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_screen(RawU16::from(color).into_inner());
        Ok(())
    }
}
