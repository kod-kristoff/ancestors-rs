use freya::{
    prelude::*,
    radio::{Radio, RadioAntenna, RadioStation},
};

use crate::app::{
    components::{EmptyPanel, StatusBar},
    panels_tabs::{PanelId, TabId, TabProps},
    state::{AppChannel, AppState},
    views::EditorView,
};

mod components;
mod panels_tabs;
mod state;
mod views;

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

        // Initialize the State Manager
        let mut radio_app_state = use_hook(move || {
            // let default_transport: FsTr

            let mut app_state = AppState::new();
            // WelcomeTab::open_with(&mut app_state);
            let station = RadioStation::create(app_state);
            provide_context(station);
            let mut radio_app_state = Radio::new(State::create(RadioAntenna::new(
                AppChannel::Global,
                station,
            )));
            radio_app_state
        });

        let (focused_view, side_panel) = {
            let app_state = radio_app_state.read();
            (app_state.focused_view, ())
        };

        let app_state_writeable = radio_app_state.slice_mut_current(|s| s).into_writable();

        let docking_area = DockingArea::new(
            app_state_writeable.clone(),
            {
                let app_state_writeable = app_state_writeable.clone();
                move |ctx: ContentContext<TabId, PanelId>| {
                    let panel_id = ctx.panel_id;
                    let Some(tab_id) = ctx.tab_id else {
                        return EmptyPanel { panel_id }.into_element();
                    };

                    let render_fn = app_state_writeable.peek().tab(&tab_id).render();
                    let content = freya::helpers::from_fn(tab_id, TabProps { tab_id }, render_fn);
                    rect()
                        .expanded()
                        .background((8, 8, 12))
                        .on_pointer_down(move |_| {
                            if radio_app_state.read().focused_panel == Some(panel_id) {
                                return;
                            }
                            let mut state = radio_app_state.write_channel(AppChannel::Global);
                            state.focused_panel = Some(panel_id);
                            if state.focused_view != EditorView::Panels {
                                state.focused_view = EditorView::Panels;
                            }
                        })
                        .child(content)
                        .into_element()
                }
            },
            {
                let app_state_writable = app_state_writable.clone();
                move |ctx: TabContext<TabId>| {
                    let tab_id = ctx.tab_id;
                    let (tab_data, is_active) = {
                        let state = app_state_writable.peek();
                        let tab_data = state.tab(&tab_id).get_data();
                        let is_active = state
                            .panel_tree
                            .as_ref()
                            .and_then(|tree| {
                                let (panel_id, _) = tree.find_tab(&tab_id)?;
                                tree.panel(&panel_id)
                                    .map(|p| p.active_tab_id == Some(tab_id))
                            })
                            .unwrap_or(false);
                        (tab_data, is_active)
                    };

                    EditorTabButton {
                        tab_id,
                        on_close: (move |_: ()| {
                            radio_app_state
                                .write_channel(AppChannel::Global)
                                .close_tab(tab_id);
                        })
                        .into(),
                        value: tab_data.title,
                        is_selected: is_active || ctx.is_drop_target,
                        icon: tab_data.icon,
                    }
                    .into_element()
                }
            },
            {
                let app_state_writable = app_state_writable.clone();
                move |tab_id: TabId| {
                    let tab_data = app_state_writable.peek().tab(&tab_id).get_data();
                    rect()
                        .interactive(false)
                        .child(EditorTabButton {
                            tab_id,
                            on_close: (|_: ()| {}).into(),
                            value: tab_data.title,
                            is_selected: true,
                            icon: tab_data.icon,
                        })
                        .into_element()
                }
            },
            move |ctx: TabBarContext<PanelId>| {
                let panel_id = ctx.panel_id;
                let children = ctx.tab_children;
                let show_close_panel = app_state_writable.peek().panels_in_order().len() > 1;

                let split_panel = move |_| {
                    radio_app_state
                        .write_channel(AppChannel::Global)
                        .split_panel(panel_id);
                };
                let close_panel = move |e: Event<PressEventData>| {
                    e.stop_propagation();
                    e.prevent_default();
                    radio_app_state
                        .write_channel(AppChannel::Global)
                        .close_panel(panel_id);
                };

                rect()
                    .horizontal()
                    .height(Size::px(32.))
                    .width(Size::fill())
                    .cross_align(Alignment::Center)
                    .content(Content::Flex)
                    .child(
                        ScrollView::new()
                            .direction(Direction::Horizontal)
                            .width(Size::flex(1.))
                            .show_scrollbar(false)
                            .child(
                                rect()
                                    .horizontal()
                                    .cross_align(Alignment::Center)
                                    .children(children),
                            ),
                    )
                    .child(
                        rect()
                            .horizontal()
                            .cross_align(Alignment::Center)
                            .main_align(Alignment::End)
                            .height(Size::fill())
                            .spacing(4.0)
                            .padding(4.0)
                            .maybe_child(show_close_panel.then(|| {
                                Button::new()
                                    .flat()
                                    .height(Size::fill())
                                    .padding((0., 8.))
                                    .on_press(close_panel)
                                    .child(
                                        svg(freya::icons::lucide::x())
                                            .width(Size::px(16.0))
                                            .height(Size::px(16.0))
                                            .color((200, 200, 200)),
                                    )
                            }))
                            .child(
                                Button::new()
                                    .flat()
                                    .height(Size::fill())
                                    .padding((0., 8.))
                                    .on_press(split_panel)
                                    .child(
                                        svg(freya::icons::lucide::columns_2())
                                            .width(Size::px(20.0))
                                            .height(Size::px(20.0))
                                            .color((200, 200, 200)),
                                    ),
                            ),
                    )
                    .into_element()
            },
        )
        .preview_element(
            rect()
                .interactive(false)
                .expanded()
                .background((255, 255, 255, 0.08)),
        );

        rect()
            .font_size(12.)
            .width(Size::fill())
            .height(Size::fill())
            .background((35, 35, 35))
            .color(Color::WHITE)
            .expanded()
            .padding(Gaps::new_all(12.))
            .child(
                rect()
                    .height(Size::func(|ctx| Some(ctx.parent - 31.)))
                    .on_mouse_up(move |_| *count.write() += 1)
                    .child(format!("Click to increase -> {}", count.read())),
            )
            .child(StatusBar { focused_view })
    }
}
