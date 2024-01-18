use std::fmt::Alignment;

use fltk::{
    button::Button,
    enums::Color,
    frame::Frame,
    group::{experimental::Grid, Flex, Pack, PackType, Scroll},
    input::Input,
    prelude::{GroupExt, InputExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
    valuator::ScrollbarType,
    window::Window,
};
pub struct MainWindow {}
impl MainWindow {
    pub fn new() -> Self {
        let mut code_list: Vec<&str> = vec!["IDI", "VIX", "HAG"];
        let mut wind = Window::default();
        wind.set_size(1000, 800);
        wind.set_label("VNINDEX");
        wind.resizable(&wind);

        let component = Pack::default()
            .size_of_parent()
            .with_type(PackType::Vertical);
        let mut app_bar = AppBar::new();
        app_bar.set_size(component.w(), 32);
        let mut code_list_cpn = Scroll::default().with_size(300, wind.h());
        code_list_cpn.set_color(Color::Red);

        let mut cols = Pack::default();
        cols.set_spacing(10);
        cols.set_size(code_list_cpn.w() - 8, component.h());
        cols.end();

        code_list_cpn.end();

        component.end();
        wind.end();
        wind.show();

        wind.center_screen();

        code_list_cpn.set_scrollbar_size(7);
        code_list_cpn.make_resizable(false);

        let mut scrollbar = code_list_cpn.scrollbar();
        scrollbar.set_type(ScrollbarType::Vertical);
        scrollbar.set_color(Color::from_u32(0x757575));
        scrollbar.set_selection_color(Color::Gray0);

        for i in code_list {
            let item_cpn = Pack::default().with_size(code_list_cpn.w() - 10, 30);
            let mut btn = Button::default().size_of_parent().with_label(i);
            btn.set_color(Color::White);
            cols.add(&item_cpn);
        }
        let mut btn_mini_view = Button::new(0, 0, code_list_cpn.w() - 10, 30, "Open mini view");
        cols.add(&btn_mini_view);
        btn_mini_view.set_callback(|b| {
            b.set_label("Close");
        });

        Self {}
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
        let mut grid = Grid::default_fill().size_of_parent();
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
            let input = self.input.clone();
            move |_| {
                println!("Occupation: {}", input.value());
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
