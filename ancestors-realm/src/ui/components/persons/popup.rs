//! # Popup

use super::{Msg, SessionMsg};

use crate::contrib::components::{Form, FormElement};
use tui_realm_stdlib::{Input, Paragraph, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

// -- error popup

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Red)
                .text(&[TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Session(SessionMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SessionOverPopup {
    component: Paragraph,
}

impl Default for SessionOverPopup {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Red)
                .text(&[TextSpan::from("You're dead. Session over!")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for SessionOverPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Session(SessionMsg::SessionOver)),
            _ => None,
        }
    }
}

// -- quit popup

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl QuitPopup {
    pub fn new() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Magenta)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Magenta)
                .title("Quit donmaze?", Alignment::Center)
                .rewind(true)
                .choices(&["Quit and save", "Quit without saving", "No"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Session(SessionMsg::CloseQuitPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.perform(Cmd::Submit) {
                CmdResult::Submit(State::One(StateValue::Usize(0))) => {
                    Some(Msg::Session(SessionMsg::Quit(true)))
                }
                CmdResult::Submit(State::One(StateValue::Usize(1))) => {
                    Some(Msg::Session(SessionMsg::Quit(false)))
                }
                _ => Some(Msg::Session(SessionMsg::CloseQuitPopup)),
            },
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SaveFileNamePopup {
    component: Input,
}

impl SaveFileNamePopup {
    pub fn new() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::LightRed)
                .placeholder("save name", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Save game as…", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for SaveFileNamePopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::String(i)) => Some(Msg::Session(SessionMsg::SaveSession(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Session(SessionMsg::CloseSaveFileName))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct AddPersonPopup {
    component: FormElement,
}

impl AddPersonPopup {
    pub fn new() -> Self {
        Self {
            component: FormElement::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::LightRed)
                .default_layout()
                // .name("name")
                .label("name")
                .placeholder("name", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Add new person…", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for AddPersonPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::String(name)) => {
                    Some(Msg::Session(SessionMsg::AddPerson(name)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Session(SessionMsg::CloseAddPersonPopup))
            }
            _ => None,
        }
    }
}
