//! # Menu
//!
//! Menu components

mod buttons;
mod input;
mod title;

use crate::ui::Msg;
pub use buttons::{Exit, LoadDb, NewDb};
pub use input::Seed;
pub use title::Title;

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MenuId {
    Title,
    NewDb,
    LoadDb,
    Seed,
    Exit,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum MenuMsg {
    ActiveNewDb,
    ActiveLoadDb,
    ActiveExit,
    ActiveSeed,
    NewDb,
    LoadDb,
    Quit,
}
