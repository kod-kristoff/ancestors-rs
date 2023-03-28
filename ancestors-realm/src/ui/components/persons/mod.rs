mod actions;
mod messages;
mod person_list;
mod popup;

pub use self::actions::AvailableActions;
pub use self::messages::Messages;
pub use self::person_list::PersonList;
pub use self::popup::AddPersonPopup;

use super::Msg;
use crate::app::session::Action;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SessionId {
    AvailableActions,
    PersonList,
    AddPersonPopup,
    EnemyName,
    ErrorPopup,
    SessionOverPopup,
    Inventory,
    Messages,
    PlayerHp,
    SaveFileNamePopup,
    QuitPopup,
}

/// Messages related to game
#[derive(PartialEq, Eq)]
pub enum SessionMsg {
    ActionSelected(Action),
    CloseAddPersonPopup,
    CloseErrorPopup,
    CloseInventory,
    CloseQuitPopup,
    CloseSaveFileName,
    SessionOver,
    /// If true, save game
    Quit(bool),
    SaveSession(String),
    AddPerson(String),
    ShowInventory,
    ShowSaveFileName,
    ShowQuitPopup,
    // UseItem(Item),
}
