//! # Load game
//!
//! Load game view components

mod db_files;
mod metadata;
mod popup;

use std::path::PathBuf;

use super::Msg;
pub use db_files::DbFiles;
pub use metadata::Metadata;
pub use popup::ErrorPopup;

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LoadDbId {
    Dbs,
    ErrorPopup,
    Metadata,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum LoadDbMsg {
    CloseErrorPopup,
    DbChanged(PathBuf),
    LoadDb(PathBuf),
    GoToMenu,
}
