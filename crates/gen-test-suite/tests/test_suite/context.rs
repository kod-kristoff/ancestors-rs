use gen_repo_sqlite::{
    SqliteAgentRepository, SqliteDocumentRepository, SqliteHouseholdRepository,
    SqlitePersonRepository, migrations::run_migrations, pool::DbPool,
};
use gen_services::{
    repositories::{
        SharedAgentRepository, SharedDocumentRepository, SharedHouseholdRepository,
        SharedPersonRepository,
    },
    service::Service,
};
use tempfile::TempDir;

// #[derive(Clone)]
pub struct TestContext {
    pub workdir: TempDir,
    pub service: Service,
    pub agent_repo: SharedAgentRepository,
    pub document_repo: SharedDocumentRepository,
    pub household_repo: SharedHouseholdRepository,
    pub person_repo: SharedPersonRepository,
}

impl TestContext {
    pub fn new() -> eyre::Result<Self> {
        let workdir = TempDir::new()?;
        let db_path = workdir.path().join("test.db").display().to_string();
        let db_pool = DbPool::new(&db_path)?;
        {
            let mut conn = db_pool.write()?;
            run_migrations(&mut conn).expect("test_suite/context: running migrations");
        }
        let agent_repo = SqliteAgentRepository::arc_new(db_pool.clone());
        let document_repo = SqliteDocumentRepository::arc_new(db_pool.clone());
        let household_repo = SqliteHouseholdRepository::arc_new(db_pool.clone());
        let person_repo = SqlitePersonRepository::arc_new(db_pool.clone());
        let service = Service::new(
            agent_repo.clone(),
            document_repo.clone(),
            household_repo.clone(),
            person_repo.clone(),
        );
        Ok(TestContext {
            workdir,
            service,
            agent_repo,
            document_repo,
            household_repo,
            person_repo,
        })
    }
}
