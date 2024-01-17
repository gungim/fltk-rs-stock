use fltk::app;
use fltk_theme::{color_themes, ColorTheme};
use mini_view::MiniWindow;
mod mini_view;
mod stock;
mod main_window;

#[tokio::main]
async fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let mini_win  = MiniWindow::new();
    mini_win.show();
    theme.apply();
    app.run().unwrap();
}
