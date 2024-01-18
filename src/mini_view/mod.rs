use std::thread;
use tokio::{runtime::Runtime, time::Duration};

use fltk::{
    app::{self, channel},
    enums::{Color, Event},
    frame::Frame,
    group::{Flex, Pack, PackType},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

use std::ops::{Deref, DerefMut};

use crate::api::{Item, call_api};

pub struct MiniWindow {
    wind: Window,
}
impl MiniWindow {
    pub fn new() -> Self {
        let mut wind = Window::new(0, 1600, 400, 50, "Mini view");
        wind.set_border(false);
        MiniView::new();

        wind.end();
        Self { wind }
    }

    pub fn show(mut self) {
        self.wind.show();
        self.wind.handle({
            let mut x = 0;
            let mut y = 0;
            move |w, ev| match ev {
                Event::Push => {
                    let coords = app::event_coords();
                    x = coords.0;
                    y = coords.1;
                    true
                }
                Event::Drag => {
                    w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                    true
                }
                _ => false,
            }
        });
    }
}

pub struct ItemCpn {
    flex: Flex,
}
impl ItemCpn {
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

impl Deref for ItemCpn {
    type Target = Flex;

    fn deref(&self) -> &Self::Target {
        &self.flex
    }
}

impl DerefMut for ItemCpn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flex
    }
}

#[derive(Clone)]
enum Message {
    Tick(Vec<Item>),
}

pub struct MiniView {}
impl MiniView {
    pub fn new() -> Self {
        let mut pack = Pack::default().size_of_parent().center_of_parent();
        pack.set_type(PackType::Horizontal);

        let mut cols = Flex::default().size_of_parent().row();
        cols.end();

        let (sender, receiver) = channel::<Message>();
        thread::spawn(move || loop {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let items: Vec<Item> = call_api().await;
                sender.send(Message::Tick(items));
            });
            thread::sleep(Duration::from_secs(10));
        });
        app::add_idle3(move |_| match receiver.recv() {
            Some(Message::Tick(items)) => {
                cols.clear();
                for i in items {
                    let code_name = i.sym;
                    let open_price: f32 = i.open_p.parse::<f32>().unwrap_or(0.0);
                    let close_price: f32 = i.closed_p.parse::<f32>().unwrap_or(0.0);
                    let f = ItemCpn::new(code_name.as_str(), open_price, close_price);
                    cols.add(&*f)
                }
            }
            None => {}
        });

        Self {}
    }
}
