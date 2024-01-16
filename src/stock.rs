use fltk::{enums::Color, frame::Frame, group::Flex, prelude::WidgetExt};
use std::ops::{
    Deref,
    DerefMut
};

pub struct Stock {
    flex: Flex,
}

impl Stock {
    pub fn new(code: &str, open: f32, close: f32) -> Self {
        let flex = Flex::default().row().center_of_parent();
        let mut code_label = Frame::default();
        let mut close_label = Frame::default();
        let mut diff_label = Frame::default();
        let diff = close - open;

        code_label.set_label(code);
        close_label.set_label(format!("{:.2}", close).as_str());
        diff_label.set_label(format!("{:.2}", diff).as_str());

        let color: Color;

        if open < close {
            color = Color::Green;
        } else if open > close {
            color = Color::Red;
        } else {
            color = Color::Yellow;
        }
        code_label.set_label_color(color);
        close_label.set_label_color(color);
        diff_label.set_label_color(color);

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
