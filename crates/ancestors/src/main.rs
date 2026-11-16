use ancestors::app::App;
use gpui_kit::AppContext;
use gpui_kit::WindowOptions;
use gpui_kit::assets;
use gpui_kit::base::Root;
use gpui_kit::component::TitleBar;

fn main() {
    gpui_kit::application()
        .with_assets(assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            let window_options = TitleBar::window_options();

            gpui_kit::open_window(window_options, cx, |window, cx| {
                let view = cx.new(App::new);
                // cx.new(|cx| Root::new(view, window, cx))
                view
            })
            .expect("Failed to open window");
        })
}
