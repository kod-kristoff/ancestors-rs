use ancestors_infra_json::repository::json::mem_gedcomx_repository::{
    MemGedcomxPersonRepo, SharedMemStorage,
};
use gen_services::repositories::SharedPersonRepository;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{fs, io};

#[derive(Clone, Default)]
pub struct AppContext {
    db: SharedMemStorage,
    db_path: Option<PathBuf>,
}

impl AppContext {
    pub fn db(&self) -> &SharedMemStorage {
        &self.db
    }

    pub fn person_repo(&self) -> SharedPersonRepository {
        MemGedcomxPersonRepo::arc_new(self.db().clone())
    }

    pub fn save_as(&mut self, path: &Path) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer(writer, &*self.db.0.read().unwrap()).unwrap();
        self.db_path = Some(path.into());
        Ok(())
    }

    pub fn load(&mut self, path: &Path) -> io::Result<()> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);

        self.db = SharedMemStorage(Arc::new(RwLock::new(
            serde_json::from_reader(reader).unwrap(),
        )));
        self.db_path = Some(path.into());
        Ok(())
    }
}
