use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
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
                    let code_name = i.symbol.unwrap_or("".to_string());
                    let open_price = i.open_price.unwrap_or(0.0);
                    let close_price = i.close_price.unwrap_or(0.0);
                    let f = ItemCpn::new(code_name.as_str(), open_price, close_price);
                    cols.add(&*f)
                }
            }
            None => {}
        });

        Self {}
    }
}

async fn call_api() -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    let re = Regex::new(r"\}.{0,7}\{").unwrap();
    let client = reqwest::Client::new();

    let response = client
        .get("https://s.cafef.vn/Ajax/PageNew/RealtimePricesHeader.ashx?symbols=VIX;IDI;SCR")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.text().await {
            Ok(txt) => {
                let s_split = &txt[8..txt.len() - 2];
                let a: Vec<&str> = re.split(s_split).collect();
                for item in a {
                    let item_str = "{".to_owned() + item + "}";
                    let value = typed_example(item_str.as_str());
                    match value {
                        Ok(t) => items.push(t),
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    };
    items
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    #[serde(rename = "FloorCode")]
    floor_code: Option<f32>,
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "LastTradeDate")]
    last_trade_date: Option<String>,
    #[serde(rename = "Price")]
    price: Option<f32>,
    #[serde(rename = "RefPrice")]
    ref_price: Option<f32>,
    #[serde(rename = "FloorPrice")]
    floor_price: Option<f32>,
    #[serde(rename = "CeilingPrice")]
    ceiling_price: Option<f32>,
    #[serde(rename = "Volume")]
    volume: Option<f32>,
    #[serde(rename = "Value")]
    value: Option<f32>,
    #[serde(rename = "HighPrice")]
    high_price: Option<f32>,
    #[serde(rename = "LowPrice")]
    low_price: Option<f32>,
    #[serde(rename = "AvgPrice")]
    avg_price: Option<f32>,
    #[serde(rename = "BidPrice01")]
    bid_price_01: Option<f32>,
    #[serde(rename = "BidPrice02")]
    bid_price_02: Option<f32>,
    #[serde(rename = "BidPrice03")]
    bid_price_03: Option<f32>,
    #[serde(rename = "BidVolume01")]
    bid_volume_01: Option<f32>,
    #[serde(rename = "BidVolume02")]
    bid_volume_02: Option<f32>,
    #[serde(rename = "BidVolume03")]
    vid_volume_03: Option<f32>,
    #[serde(rename = "AskVolume01")]
    ask_price_01: Option<f32>,
    #[serde(rename = "AskPrice02")]
    ask_price_02: Option<f32>,
    #[serde(rename = "AskVolume03")]
    ask_price_03: Option<f32>,
    #[serde(rename = "AskVolume01")]
    ask_volume_01: Option<f32>,
    #[serde(rename = "AskVolume02")]
    ask_volume_02: Option<f32>,
    #[serde(rename = "AskVolume03")]
    ask_volume_03: Option<f32>,
    #[serde(rename = "BidTotalVolume")]
    bid_total_volume: Option<f32>,
    #[serde(rename = "BidTotalOrder")]
    bid_total_order: Option<f32>,
    #[serde(rename = "AskTotalVolume")]
    ask_total_volume: Option<f32>,
    #[serde(rename = "AskTotalOrder")]
    ask_total_order: Option<f32>,
    #[serde(rename = "OpenPrice")]
    open_price: Option<f32>,
    #[serde(rename = "ClosePrice")]
    close_price: Option<f32>,
    #[serde(rename = "ForeignBuyVolume")]
    foreign_buy_volume: Option<f32>,
    #[serde(rename = "ForeignBuyValue")]
    foreign_buy_value: Option<f32>,
    #[serde(rename = "ForeignSellVolume")]
    foreign_sell_volume: Option<f32>,
    #[serde(rename = "ForeignSellValue")]
    foreign_sell_value: Option<f32>,
    #[serde(rename = "ForeignNetVolume")]
    foreign_net_volume: Option<f32>,
    #[serde(rename = "ForeignCurrentRoom")]
    foreign_current_room: Option<f32>,
    #[serde(rename = "ForeignTotalRoom")]
    foreign_total_room: Option<f32>,
    #[serde(rename = "LastVolume")]
    last_volume: Option<f32>,
}

fn typed_example(data: &str) -> serde_json::Result<Item> {
    let p: Item = from_str(data)?;
    Ok(p)
}
