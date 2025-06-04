use embedded_graphics::{
    mono_font::{iso_8859_3::FONT_6X9, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::PrimitiveStyleBuilder,
    text::{Alignment, Text},
};

pub trait DrawableUi {
    type Error;
    fn draw_menu(&mut self, items: &Vec<MenuItem>) -> Result<(), Self::Error>;
    fn draw_menu_item(&mut self, dx: i32, dy: i32, item: &MenuItem) -> Result<(), Self::Error>;
    fn draw_menu_list(&mut self) -> Result<(), Self::Error>;
    fn draw_menu_list_item(&mut self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct MenuItem<'a> {
    pub title: &'a str,
    pub selected: bool,
}

#[derive(Debug)]
pub struct Menu<'a> {
    active: usize,
    items: Vec<MenuItem<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new(items: Vec<MenuItem<'a>>) -> Self {
        Self { active: 0, items }
    }

    pub fn prev_item(&mut self) -> Option<&MenuItem> {
        if let Some(item) = self.items.get_mut(self.active) {
            item.selected = false;
        }

        if self.active == 0 {
            self.active = self.items.len() - 1;
            let item = self.items.get_mut(self.active).unwrap();
            item.selected = true;
            return Some(item);
        } else if let Some(item) = self.items.get_mut(self.active - 1) {
            self.active -= 1;
            item.selected = true;
            return Some(item);
        }

        None
    }

    pub fn next_item(&mut self) -> Option<&MenuItem> {
        if let Some(item) = self.items.get_mut(self.active) {
            item.selected = false;
        }

        if self.active + 1 >= self.items.len() {
            self.active = 0;
            let item = self.items.get_mut(0).unwrap();
            item.selected = true;
            return Some(item);
        } else if let Some(item) = self.items.get_mut(self.active + 1) {
            self.active += 1;
            item.selected = true;
            return Some(item);
        }

        None
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        display.draw_menu(self.items.as_ref())?;
        Ok(())
    }
}

impl<D> DrawableUi for D
where
    D: DrawTarget<Color = Rgb565>,
{
    type Error = D::Error;

    fn draw_menu(&mut self, items: &Vec<MenuItem>) -> Result<(), Self::Error> {
        // FIXME: not hardcoded values
        let display_width = 240;
        let display_height = 320;

        let columns = 2;
        let grid_padding_x = 16;
        let grid_padding_y = 18;
        let cell_padding_x = 16;
        let cell_padding_y = 18;

        let available_width = display_width - 2 * grid_padding_x;
        let available_height = display_height - 2 * grid_padding_y;

        let spacing_x = available_width / columns;
        let rows = (items.len() + columns - 1) / columns;
        let spacing_y = available_height / rows;

        self.clear(Rgb565::BLACK)?;
        for (i, item) in items.into_iter().enumerate() {
            let col = i % columns;
            let row = i / columns;

            let x = grid_padding_x + col * spacing_x + cell_padding_x;
            let y = grid_padding_y + row * spacing_y + cell_padding_y;

            self.draw_menu_item(x as i32, y as i32, item)?;
        }
        Ok(())
    }

    fn draw_menu_item(&mut self, dx: i32, dy: i32, item: &MenuItem) -> Result<(), Self::Error> {
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(if item.selected {
                Rgb565::CSS_LIGHT_GRAY
            } else {
                Rgb565::CSS_DARK_CYAN
            })
            .fill_color(if item.selected {
                Rgb565::CSS_LIGHT_GRAY
            } else {
                Rgb565::CSS_DARK_CYAN
            })
            .build();
        let character_style = MonoTextStyle::new(&FONT_6X9, Rgb565::BLACK);

        let text = Text::with_alignment(
            item.title,
            Point::new(dx, dy),
            character_style,
            Alignment::Left,
        );

        text.bounding_box().into_styled(style).draw(self)?;

        text.draw(self)?;
        Ok(())
    }

    fn draw_menu_list(&mut self) -> Result<(), Self::Error> {
        let _title = "Control";
        todo!()
    }

    fn draw_menu_list_item(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
