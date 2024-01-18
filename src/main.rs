use fltk::app;
use fltk_theme::{color_themes, ColorTheme};
use main_window::MainWindow;
use mini_view::MiniWindow;
mod main_window;
mod mini_view;
mod stock;

#[tokio::main]
async fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);

    let mini_win  = MiniWindow::new();
    mini_win.show();
    // let _main_window = MainWindow::new();
    theme.apply();
    app.run().unwrap();
}
