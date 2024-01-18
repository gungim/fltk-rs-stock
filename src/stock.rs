use fltk::{group::Flex, prelude::WidgetExt};
use std::ops::{Deref, DerefMut};

pub struct Stock {
    flex: Flex,
}

impl Stock {
    pub fn new() -> Self {
        let flex = Flex::default().row().center_of_parent();

        Self { flex }
    }
}

impl Deref for Stock {
    type Target = Flex;

    fn deref(&self) -> &Self::Target {
        &self.flex
    }
}

impl DerefMut for Stock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flex
    }
}
