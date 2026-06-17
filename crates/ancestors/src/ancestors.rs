mod app_menus;

use ancestors_actions::Quit;
use gpui::App;

pub use app_menus::app_menus;

pub fn init(cx: &mut App) {
    cx.on_action(quit);
}

fn quit(_: &Quit, cx: &mut App) {
    println!("Gracefully quitting the application...");
    cx.quit();
}
