use fltk::{
    button::Button,
    enums::Color,
    frame::Frame,
    group::{experimental::Grid, Flex, Pack, PackType, Scroll},
    input::Input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
    valuator::ScrollbarType,
    window::Window,
};

use crate::{database::StockDB, mini_view::MiniWindow};

pub struct MainWindow {
    whitelist_wg: Pack,
    whitelist: Vec<String>,
    wind: Window,
    btn_mini_w: Button,
}

impl MainWindow {
    pub fn new() -> Self {
        let whitelist = vec![];
        let mut wind = Window::default();
        wind.set_size(1000, 800);
        wind.set_label("VNINDEX");
        wind.resizable(&wind);

        let component = Pack::default()
            .size_of_parent()
            .with_type(PackType::Vertical);
        let mut app_bar = AppBar::new();
        app_bar.set_size(component.w(), 32);

        let grid = Pack::default()
            .size_of_parent()
            .with_type(PackType::Horizontal);

        let mut side_bar_wg = Pack::default().with_size(200, wind.h());
        side_bar_wg.set_color(Color::Red);

        let mut side_bar_scroll = Scroll::default().size_of_parent();
        side_bar_scroll.set_scrollbar_size(7);
        side_bar_scroll.set_color(Color::Red);
        let btn_mini_w = Button::default()
            .with_size(200, 32)
            .with_label("Open mini view");

        let mut whitelist_wg = Pack::default();
        whitelist_wg.set_spacing(5);
        whitelist_wg.set_size(side_bar_wg.w() - 8, component.h());
        whitelist_wg.end();

        side_bar_wg.end();

        grid.end();
        component.end();

        wind.end();

        let mut scrollbar = side_bar_scroll.scrollbar();
        scrollbar.set_type(ScrollbarType::Vertical);
        scrollbar.set_color(Color::from_u32(0x757575));
        scrollbar.set_selection_color(Color::Gray0);

        let mut g = Self {
            whitelist_wg,
            whitelist,
            wind,
            btn_mini_w,
        };
        g.update_whitelist();
        g.open_mini_w();
        g
    }

    pub fn update_whitelist(&mut self) {
        self.whitelist = StockDB::get_codes();
        for i in &self.whitelist {
            let btn = Button::new(0, 0, 280, 32, i.to_string().as_str());
            self.whitelist_wg.add(&btn)
        }
    }

    pub fn show(mut self) {
        self.wind.show();
        self.wind.center_screen();
    }
    pub fn hide(mut self){
        self.wind.hide();

    }

    pub fn open_mini_w(&mut self) {
        self.btn_mini_w.set_callback(move |_| {
            let mini_w = MiniWindow::new();
            mini_w.show();
            self.hide();
        })
    }
}

pub struct AppBar {
    grid: Grid,
    img: Frame,
    input: Input,
    btn: Button,
}
impl AppBar {
    pub fn new() -> Self {
        let mut grid = Grid::default();
        let img = Frame::default().with_label("VNINDEX");
        let input = Input::default();
        let btn = Button::default().with_label("Search");
        grid.end();
        grid.set_layout(1, 12);
        grid.set_gap(10, 10);

        let mut g = Self {
            grid,
            img,
            input,
            btn,
        };
        g.fill();
        g.register_default_callback();
        g
    }

    fn register_default_callback(&mut self) {
        self.btn.set_callback({
            move |_| {
                let data = StockDB::get_codes();
                for i in data {
                    println!("{:?}", i)
                }
            }
        });
    }

    fn fill(&mut self) {
        let grid = &mut self.grid;
        grid.set_margin(0, 3, 0, 3);
        grid.show_grid(false);
        grid.set_widget(&mut self.img, 0, 1);
        grid.set_widget(&mut self.input, 0, 2..9);
        grid.set_widget(&mut self.btn, 0, 10);
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.grid.set_size(w, h);
    }
}
