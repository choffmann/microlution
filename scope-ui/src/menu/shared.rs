use core::cell::RefCell;
use std::fmt::Write;
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use embedded_menu::items::menu_item::SelectValue;

use super::position::Position;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Shared<T> {
    value: Rc<RefCell<T>>,
    cache: RefCell<String>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared {
            value: self.value.clone(),
            cache: RefCell::new(String::new()),
        }
    }
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Self {
        Shared {
            value: Rc::new(RefCell::new(inner)),
            cache: RefCell::new(String::new()),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }
}

impl SelectValue for Shared<Position> {
    fn marker(&self) -> &str {
        let mut cache = self.cache.borrow_mut().clone();
        cache.clear();
        cache = self.borrow().string_repr();
        self.cache.replace(cache);

        ""
    }
}
