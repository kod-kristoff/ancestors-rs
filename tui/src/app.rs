#[derive(Debug, Clone)]
pub struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl App {
    pub fn new() -> Self {
        log::trace!("creating App ...");
        Self::default()
    }
}
