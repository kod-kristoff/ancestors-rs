use ancestors::{domain::GedcomX, infrastructure::SharedGedcomX};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{fs, io};

#[derive(Clone)]
pub struct AppContext {
    db: SharedGedcomX,
    db_path: Option<PathBuf>,
    pub state: AppState,
}

#[derive(Clone)]
pub enum AppState {
    MainView,
    EditPerson(String),
}

impl Default for AppState {
    fn default() -> Self {
        Self::MainView
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            db: Arc::new(RwLock::new(GedcomX::new())),
            db_path: None,
            state: AppState::default(),
        }
    }
}

impl AppContext {
    pub fn db(&self) -> &SharedGedcomX {
        &self.db
    }

    pub fn save_as(&mut self, path: &Path) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer(writer, &*self.db.read().unwrap()).unwrap();
        self.db_path = Some(path.into());
        Ok(())
    }

    pub fn load(&mut self, path: &Path) -> io::Result<()> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);

        self.db = Arc::new(RwLock::new(serde_json::from_reader(reader).unwrap()));
        self.db_path = Some(path.into());
        Ok(())
    }
}
