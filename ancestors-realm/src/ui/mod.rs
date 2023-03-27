use std::path::PathBuf;
use std::time::Duration;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::{
    props::Shape, terminal::TerminalBridge, Application, EventListenerCfg, NoUserEvent,
};

mod components;
mod error;

use crate::app::Session;

use self::components::load_db;
pub use self::components::{
    load_db::{LoadDbId, LoadDbMsg},
    menu::{MenuId, MenuMsg},
};
pub use self::error::UiError;

use self::components::{common::draw_area_in, menu};

pub type UiResult<T> = Result<T, UiError>;

/// Application ID
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    LoadDb(LoadDbId),
    Menu(MenuId),
}

/// Application MSG
#[derive(PartialEq, Eq)]
pub enum Msg {
    LoadDb(LoadDbMsg),
    Menu(MenuMsg),
    None,
}
/// Current UI view
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum View {
    LoadDb,
    Menu,
    None,
}

/// Ancestors TUI
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    terminal: TerminalBridge,
    view: View,
}

impl Ui {
    /// Create Ui
    pub fn new() -> UiResult<Self> {
        log::trace!("creating ui");
        let application = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        Ok(Self {
            application,
            terminal: TerminalBridge::new()?,
            view: View::None,
        })
    }

    /// Tick application events
    pub fn tick(&mut self) -> UiResult<Vec<Msg>> {
        let msg = self.application.tick(tuirealm::PollStrategy::UpTo(3))?;
        Ok(msg)
    }

    /// Init terminal
    pub fn init_terminal(&mut self) {
        let _ = self.terminal.enable_raw_mode();
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    /// Finalize terminal
    pub fn finalize_terminal(&mut self) {
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    pub fn sizes(&self) -> UiResult<(u16, u16)> {
        self.terminal
            .raw()
            .size()
            .map_err(|_| UiError::FailedToGetSize)
            .map(|rect| (rect.width, rect.height))
    }
    // -- @! view loaders
    /// load session loader
    pub fn load_db_loader(
        &mut self,
        sessions: &[PathBuf],
        session0: Option<&Session>,
    ) -> UiResult<()> {
        self.application.umount_all();
        self.application.mount(
            Id::LoadDb(LoadDbId::Dbs),
            Box::new(load_db::DbFiles::new(sessions)),
            vec![],
        )?;
        if let Some(session) = session0 {
            self.application.mount(
                Id::LoadDb(LoadDbId::Metadata),
                Box::new(load_db::Metadata::new(session)),
                vec![],
            )?;
        }
        self.application.active(&Id::LoadDb(LoadDbId::Dbs))?;
        self.view = View::LoadDb;
        Ok(())
    }

    /// Load menu view
    pub fn load_menu(&mut self) -> UiResult<()> {
        self.application.umount_all();
        let (width, _) = self.sizes()?;
        let width = width as u16 - 60;
        log::debug!("mounting Title");
        self.application.mount(
            Id::Menu(MenuId::Title),
            Box::new(menu::Title::new(width)),
            vec![],
        )?;
        log::debug!("mounting NewDb");
        self.application.mount(
            Id::Menu(MenuId::NewDb),
            Box::new(menu::NewDb::default()),
            vec![],
        )?;
        log::debug!("mounting Seed");
        self.application.mount(
            Id::Menu(MenuId::Seed),
            Box::new(menu::Seed::default()),
            vec![],
        )?;
        log::debug!("mounting LoadDb");
        self.application.mount(
            Id::Menu(MenuId::LoadDb),
            Box::new(menu::LoadDb::default()),
            vec![],
        )?;
        log::debug!("mounting Exit");
        self.application.mount(
            Id::Menu(MenuId::Exit),
            Box::new(menu::Exit::default()),
            vec![],
        )?;
        log::debug!("mounting title");
        self.application.active(&Id::Menu(MenuId::NewDb))?;
        self.view = View::Menu;
        Ok(())
    }

    /// Active focus
    pub fn active(&mut self, id: Id) {
        let _ = self.application.active(&id);
    }

    // @! view

    /// Display ui to terminal
    pub fn draw(&mut self) -> UiResult<()> {
        match self.view {
            // View::Db => self.view_session(),
            // View::DbOver => self.view_session_over(),
            View::LoadDb => self.draw_load_db(),
            View::Menu => self.draw_menu(),
            // View::Victory => self.view_victory(),
            View::None => Ok(()),
        }
    }

    fn draw_load_db(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(10)
                .constraints(
                    [
                        Constraint::Percentage(60), // List
                        Constraint::Percentage(40), // metadata
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let metadata_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(10), // metadata
                ])
                .split(body[1]);
            self.application
                .view(&Id::LoadDb(LoadDbId::Dbs), f, body[0]);
            self.application
                .view(&Id::LoadDb(LoadDbId::Metadata), f, metadata_chunks[0]);
            // popups
            if self.application.mounted(&Id::LoadDb(LoadDbId::ErrorPopup)) {
                let popup = draw_area_in(f.size(), 50, 20);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::LoadDb(LoadDbId::ErrorPopup), f, popup);
            }
        })?;
        Ok(())
    }

    fn draw_menu(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(10)
                .constraints(
                    [
                        Constraint::Length(7), // Title
                        Constraint::Length(3), // new session + seed
                        Constraint::Length(3), // load session
                        Constraint::Length(3), // quit
                        Constraint::Length(1), // footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let new_db_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(body[1]);
            self.application.view(&Id::Menu(MenuId::Title), f, body[0]);
            self.application
                .view(&Id::Menu(MenuId::NewDb), f, new_db_chunks[0]);
            self.application
                .view(&Id::Menu(MenuId::Seed), f, new_db_chunks[1]);
            self.application.view(&Id::Menu(MenuId::LoadDb), f, body[2]);
            self.application.view(&Id::Menu(MenuId::Exit), f, body[3]);
        })?;
        Ok(())
    }
}
