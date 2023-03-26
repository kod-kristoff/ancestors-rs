use crate::app::AppResult;
use crate::ui::{Id, MenuId, MenuMsg, Msg, Ui};

/// App runtime
pub struct Runtime {
    ui: Ui,
    running: bool,
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
        Ok(Self { ui, running: true })
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
            Msg::Menu(msg) => self.update_menu(msg),
        }
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
                todo!("load db")
                // let saved_games = SavedDbFiles::saved_games(&self.saved_games_dir)?;
                // if saved_games.is_empty() {
                //     self.play_sound(Sound::Error);
                //     self.play_sound(Sound::Input);
                // } else {
                //     let game_0 = match saved_games.get(0) {
                //         None => None,
                //         Some(p) => SavedDbFiles::load_game(p).ok(),
                //     };
                //     self.ui.load_game_loader(&saved_games, game_0.as_ref())?;
                // }
            }
            MenuMsg::NewDb => {
                // create a new session
                // let seed = self.ui.get_menu_seed()?;
                let seed = 0;
                log::debug!("initializing new session with seed {:?}", seed);
                // self.start_maze(Session::new(seed))?;
            }
            MenuMsg::Quit => {
                self.running = false;
            }
        }
        Ok(())
    }
}
