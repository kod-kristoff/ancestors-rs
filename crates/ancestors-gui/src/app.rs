use std::rc::Rc;

use freya::prelude::*;

#[derive(PartialEq)]
pub struct AppView {}

impl AppView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        launch(
            LaunchConfig::new().with_window(WindowConfig::new_app(self).with_title("Ancestors")),
        );
    }
}

impl App for AppView {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| 0);
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((35, 35, 35))
            .color(Color::WHITE)
            .padding(Gaps::new_all(12.))
            .on_mouse_up(move |_| *count.write() += 1)
            .child(format!("Click to increase -> {}", count.read()))
            .into_element()
    }
}
