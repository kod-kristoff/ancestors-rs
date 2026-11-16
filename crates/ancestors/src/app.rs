use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::ContextMenuExt;
use gpui_kit::component::text::TextView;
use gpui_kit::component::{TitleBar, WindowExt};
use gpui_kit::*;

actions!(class_menu, [Open, Delete, Export, Info]);

#[derive(Default)]
struct AppModel {
    persons: Vec<SharedString>,
}
pub struct App {
    store: Entity<AppModel>,
    _subscription: Subscription,
}

impl App {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let store = cx.new(|_| AppModel {
            persons: Vec::new(),
        });
        let _subscription = cx.observe(&store, |_, _, cx| cx.notify());
        Self {
            store,
            _subscription,
        }
    }
    fn add(&mut self, cx: &mut Context<Self>) {
        self.store.update(cx, |store, cx| {
            let id = store.persons.len();
            store.persons.push(format!("Person {id}").into());
            cx.notify();
        });
    }

    fn show_dialog(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.open_dialog(cx, move |dialog, _, _| {
            dialog.title("Selectable dialog").child(
                TextView::markdown(
                    "dialog-text",
                    "Select **this** text, then drag the mouse *out of the dialog* over \
                       the paragraph behind it. The text behind must NOT get selected",
                )
                .selectable(true),
            )
        });
    }

    fn show_sheet(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.open_sheet(cx, move |sheet, _, _| {
            sheet.title("Selectable Sheet").child(
                TextView::markdown(
                    "sheet-text",
                    "Select **this** text, then drag the mouse *out of the sheet* over \
                       the paragraph behind it. The text behind must NOT get selected",
                )
                .selectable(true),
            )
        });
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active = window.is_window_active();
        div()
            .size_full()
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .pr_2()
                        .justify_between()
                        .child("Ancestors")
                        .child("Right Item"),
                ),
            )
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .child(self.sidebar(cx))
                    .child(self.main_content(cx)),
            )
    }
}

// Render helpers for App
impl App {
    fn main_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.store.read(cx).persons.len();
        div()
            .p_8()
            .flex()
            .flex_col()
            .gap_2()
            .size_full()
            .child(format!("Persons: {count}"))
            // Selectable text behind the modals. Open a dialog/sheet,
            // select its text, drag the mouse out over this paragraph,
            // and confirm this text is NOT selected.
            .child(
                Button::new("add-person")
                    .label("Add person")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.store.update(cx, |store, cx| {
                            let id = 1900 + store.persons.len();
                            store
                                .persons
                                .push(format!("Andersson, Kristoffer ({id} -)").into());
                            cx.notify();
                        })
                    })),
            )
            .child(
                div()
                    .id("second-area")
                    .v_flex()
                    .h_40()
                    .border_1()
                    .border_dashed()
                    .border_color(gpui_kit::black())
                    .items_center()
                    .justify_center()
                    .hover(|this| this.bg(gpui_kit::yellow().opacity(0.2)))
                    .child("Hover test here.")
                    .child("Right click to show Context Menu")
                    .context_menu({
                        move |this, _, _| {
                            this.separator()
                                .menu("Open", Box::new(Open))
                                .menu("Delete", Box::new(Delete))
                                .menu("Export", Box::new(Export))
                                .menu("Info", Box::new(Info))
                                .separator()
                        }
                    }),
            )
    }

    fn sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let persons = self.store.read(cx).persons.iter().cloned();
        div()
            .w_1_4()
            .h_full()
            .flex()
            .flex_col()
            .border_r_1()
            .px_2()
            .py_2()
            .gap_1()
            .children(persons.map(|person| div().child(person)))
    }
}
