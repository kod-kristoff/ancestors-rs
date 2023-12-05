use std::path::PathBuf;

use ancestors_core::application::service::AddPerson;

use crate::app::session::Action;
use crate::app::{AppResult, Session};
use crate::serialization::sessions::SavedSessions;
use crate::ui::{Id, LoadDbId, LoadDbMsg, MenuId, MenuMsg, Msg, SessionMsg, Ui};

/// App runtime
pub struct Runtime {
    ui: Ui,
    saved_sessions_dir: PathBuf,
    running: bool,
    session: Option<Session>,
}

impl Runtime {
    /// Setup app runtime
    pub fn setup() -> AppResult<Self> {
        log::debug!("setting up app runtime");
        log::debug!("initializing ui");
        let mut ui = Ui::new()?;
        log::info!("Ui OK");
        log::debug!("loading menu");
        ui.load_menu()?;
        log::info!("menu loaded");
        Ok(Self {
            ui,
            saved_sessions_dir: "sessions".into(),
            running: true,
            session: None,
        })
    }

    /// Run app
    pub fn run(mut self) -> AppResult<()> {
        log::trace!("run");
        log::debug!("initializing terminal...");
        self.ui.init_terminal();
        log::debug!("starting main loop ...");
        let mut redraw = true;
        while self.running {
            let messages = self.ui.tick()?;
            for msg in messages.into_iter() {
                self.update(msg)?;
                redraw = true;
            }
            if redraw {
                self.ui.draw()?;
                redraw = false;
            }
        }
        log::debug!("finalizing terminal...");
        self.ui.finalize_terminal();
        Ok(())
    }

    /// Custom implementation of the Update trait
    fn update(&mut self, msg: Msg) -> AppResult<()> {
        match msg {
            Msg::None => Ok(()),
            Msg::LoadDb(msg) => self.update_load_db(msg),
            Msg::Menu(msg) => self.update_menu(msg),
            Msg::Session(msg) => self.update_session(msg),
        }
    }

    fn update_load_db(&mut self, msg: LoadDbMsg) -> AppResult<()> {
        match msg {
            LoadDbMsg::CloseErrorPopup => {
                // self.ui.close_load_game_error()?;
            }
            LoadDbMsg::GoToMenu => {
                self.ui.load_menu()?;
            }
            LoadDbMsg::DbChanged(p) => todo!("db changed"),
            // match SavedDbFiles::load_game(&p) {
            //     Err(e) => {
            //         log::error!("failed to load game: {}", e);
            //         // self.ui
            //         //     .show_load_game_error(format!("failed to load game: {}", e))?;
            //     }
            //     Ok(session) => {
            //         // self.ui.set_load_game_save_metadata(&session)?;
            //     }
            // },
            LoadDbMsg::LoadDb(game_file) => {
                // self.load_game(&game_file)?;
            }
        }
        Ok(())
    }

    fn update_menu(&mut self, msg: MenuMsg) -> AppResult<()> {
        match msg {
            MenuMsg::ActiveExit => {
                self.ui.active(Id::Menu(MenuId::Exit));
            }
            MenuMsg::ActiveLoadDb => {
                self.ui.active(Id::Menu(MenuId::LoadDb));
            }
            MenuMsg::ActiveNewDb => {
                self.ui.active(Id::Menu(MenuId::NewDb));
            }
            MenuMsg::ActiveSeed => {
                self.ui.active(Id::Menu(MenuId::Seed));
            }
            MenuMsg::LoadDb => {
                let saved_sessions = SavedSessions::saved_sessions(&self.saved_sessions_dir)?;
                if saved_sessions.is_empty() {
                    log::error!("No saved sessions");
                    // self.play_sound(Sound::Error);
                    // self.play_sound(Sound::Input);
                } else {
                    let session_0 = match saved_sessions.get(0) {
                        None => None,
                        Some(p) => SavedSessions::load_session(p).ok(),
                    };
                    self.ui
                        .load_db_loader(&saved_sessions, session_0.as_ref())?;
                }
            }
            MenuMsg::NewDb => {
                // create a new session
                // let seed = self.ui.get_menu_seed()?;
                // let seed = 0;
                log::debug!("initializing new session");
                self.start_session(Session::default())?;
            }
            MenuMsg::Quit => {
                self.running = false;
            }
        }
        Ok(())
    }

    fn update_session(&mut self, msg: SessionMsg) -> AppResult<()> {
        match msg {
            SessionMsg::ActionSelected(action) => match action {
                Action::AddPerson => self.ui.show_add_person_popup()?,
                _ => todo!(),
            },
            SessionMsg::CloseAddPersonPopup => todo!(),
            SessionMsg::AddPerson(name) => {
                let cmd: AddPerson = AddPerson {
                    name: Some(name),
                    ..Default::default()
                };
                let repo = self
                    .session
                    .as_ref()
                    .expect("runtime: session should exist")
                    .get_person_repo();
                todo!(
                    "let uc = AddingPerson::new(repo);
                uc.execute(&cmd)?;"
                );
            }
            SessionMsg::CloseErrorPopup => todo!(),
            SessionMsg::CloseInventory => todo!(),
            SessionMsg::CloseQuitPopup => todo!(),
            SessionMsg::CloseSaveFileName => todo!(),
            SessionMsg::SessionOver => todo!(),
            SessionMsg::SaveSession(string) => todo!(),
            SessionMsg::ShowInventory => todo!(),
            SessionMsg::ShowSaveFileName => todo!(),
            SessionMsg::ShowQuitPopup => todo!(),
            SessionMsg::Quit(should_save) => {}
        }
        Ok(())
    }

    fn start_session(&mut self, session: Session) -> AppResult<()> {
        self.ui.load_session(&session)?;
        self.session = Some(session);
        Ok(())
    }
}
