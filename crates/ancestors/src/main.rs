use crate::ancestors::app_menus;
use gpui::{
    App, AppContext, Application, Bounds, ParentElement, Render, SharedString, Styled,
    WindowBounds, WindowOptions, div, px, rgb, size,
};

mod ancestors;

fn main() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        anc_menu::init();
        ancestors_actions::init();

        ancestors::init(cx);

        cx.activate(true);
        let menus = app_menus(cx);
        cx.set_menus(menus);

        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(gpui::white())
            .size_full()
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(gpui::black())
            .child(format!("Hello, {}!", &self.text))
    }
}
