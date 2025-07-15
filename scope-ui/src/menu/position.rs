use std::ops::{Add, Sub};

use embedded_menu::items::menu_item::SelectValue;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Position {
    value: i64,
    string_repr: String,
}

impl Position {
    pub fn new(value: i64) -> Self {
        let string_repr = value.to_string();
        Self { value, string_repr }
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
        self.string_repr = value.to_string();
    }

    pub fn string_repr(&self) -> String {
        self.string_repr.clone()
    }
}

impl From<i64> for Position {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Add<i64> for Position {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Self::new(self.value + rhs)
    }
}

impl Sub<i64> for Position {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        Self::new(self.value - rhs)
    }
}

impl SelectValue for Position {
    fn marker(&self) -> &str {
        &self.string_repr
    }
}

impl SelectValue for &Position {
    fn marker(&self) -> &str {
        self.string_repr.as_ref()
    }
}
