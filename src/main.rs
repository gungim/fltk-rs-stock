use database::StockDB;
use fltk::app;
use fltk_theme::{color_themes, ColorTheme};
use main_window::MainWindow;
// use mini_view::MiniWindow;

mod api;
mod database;
mod main_window;
mod mini_view;
mod stock;
mod test_state;

#[tokio::main]
async fn main() {
    let _ = StockDB::new();
    let codes = vec!["VIX", "HAG", "ACB", "ITA"];

    for i in codes {
        StockDB::add_code(i.to_string())
    }

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);


    let main_window = MainWindow::new();
    main_window.show();

    theme.apply();
    app.run().unwrap();
}
