use std::collections::HashMap;
use std::{env, process};

use cosmic::app::{self, Core, Task};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::keyboard::Key;
use cosmic::iced::{Length, window};
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::menu::key_bind::{KeyBind, Modifier};
use cosmic::widget::menu::{ItemHeight, ItemWidth};
use cosmic::widget::{RcElementWrapper, menu};
use cosmic::{Element, executor};

#[derive(Debug, Clone)]
pub struct Flags {}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    FamilyTreeNew,
    FamilyTreeOpen,
    WindowClose,
    WindowNew,
    ToggleHideContent,
    Cosmic(app::Action),
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    config: Config,
    key_binds: HashMap<KeyBind, Action>,
}

pub struct Config {
    hide_content: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    FamilyTreeNew,
    FamilyTreeOpen,
    WindowClose,
    ToggleHideContent,
    WindowNew,
}

impl MenuAction for Action {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            Action::FamilyTreeNew => Message::FamilyTreeNew,
            Action::FamilyTreeOpen => Message::FamilyTreeOpen,
            Action::WindowClose => Message::WindowClose,
            Action::ToggleHideContent => Message::ToggleHideContent,
            Action::WindowNew => Message::WindowNew,
        }
    }
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = Flags;

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.AppDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits task on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = App {
            core,
            config: Config {
                hide_content: false,
            },
            key_binds: key_binds(),
        };

        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        vec![menu_bar(&self.config, &self.key_binds)]
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::FamilyTreeNew => match env::current_exe() {
                Ok(exe) => match process::Command::new(&exe).spawn() {
                    Ok(_child) => {}
                    Err(err) => {
                        eprintln!("failed to execute {:?}: {}", exe, err);
                    }
                },
                Err(err) => {
                    eprintln!("failed to get current executable path: {}", err);
                }
            },
            Message::FamilyTreeOpen => todo!(),
            Message::WindowClose => {
                return window::close(self.core.main_window_id().unwrap());
            }
            Message::WindowNew => match env::current_exe() {
                Ok(exe) => match process::Command::new(&exe).spawn() {
                    Ok(_child) => {}
                    Err(err) => {
                        eprintln!("failed to execute {:?}: {}", exe, err);
                    }
                },
                Err(err) => {
                    eprintln!("failed to get current executable path: {}", err);
                }
            },
            Message::ToggleHideContent => self.config.hide_content = !self.config.hide_content,
            Message::Cosmic(cosmic) => {
                // Forward cosmic messages
                return Task::perform(async move { cosmic }, cosmic::action::cosmic);
            }
        }
        Task::none()
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<'_, Self::Message> {
        let text = if self.config.hide_content {
            cosmic::widget::text("")
        } else {
            cosmic::widget::text("Menu Example")
        };

        let centered = cosmic::widget::container(text)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        Element::from(centered)
    }
}

pub fn menu_bar<'a>(config: &Config, key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    menu::bar(vec![
        menu::Tree::with_children(
            RcElementWrapper::new(Element::from(menu::root("Ancestors"))),
            menu::items(
                key_binds,
                vec![
                    menu::Item::Divider,
                    menu::Item::Button(
                        "Quit",
                        Some(cosmic::widget::icon::from_name("window-close-symbolic").into()),
                        Action::WindowClose,
                    ),
                ],
            ),
        ),
        menu::Tree::with_children(
            RcElementWrapper::new(Element::from(menu::root("Family Tree"))),
            menu::items(
                key_binds,
                vec![
                    menu::Item::Button(
                        "New",
                        Some(cosmic::widget::icon::from_name("screenshot-window-symbolic").into()),
                        Action::FamilyTreeNew,
                    ),
                    menu::Item::Divider,
                    menu::Item::Button(
                        "Open...",
                        Some(cosmic::widget::icon::from_name("screenshot-open-symbolic").into()),
                        Action::FamilyTreeOpen,
                    ),
                    menu::Item::Folder(
                        "View",
                        vec![menu::Item::CheckBox(
                            "Hide content",
                            Some(cosmic::widget::icon::from_name("view-conceal-symbolic").into()),
                            config.hide_content,
                            Action::ToggleHideContent,
                        )],
                    ),
                ],
            ),
        ),
    ])
    .item_height(ItemHeight::Dynamic(40))
    .item_width(ItemWidth::Uniform(240))
    .spacing(4.0)
    .into()
}

pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();

    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$(Modifier::$modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }

    bind!([Ctrl], Key::Character("w".into()), WindowClose);
    bind!([Ctrl, Shift], Key::Character("n".into()), WindowNew);

    key_binds
}
