use std::{collections::HashMap, fs};

use ancestors_cli::{cli::flags, shared::pretty::prepare_and_run};
use gen_repo_sqlite::{
    migrations::run_migrations, pool::DbPool, SqliteAgentRepository, SqliteDocumentRepository,
    SqliteHouseholdRepository, SqlitePersonRepository,
};
use gen_services::service::Service;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}
fn try_main() -> miette::Result<()> {
    let flags = flags::AncestorsCli::from_env_or_exit();
    dbg!(&flags);
    let verbose = flags.verbose;
    let trace = flags.trace;

    let cmd = flags.subcommand;
    match cmd {
        flags::AncestorsCliCmd::Scrape(flags::Scrape { url }) => {
            let db_pool = DbPool::new("./data/ancestors.db3").unwrap();
            {
                let mut conn = db_pool.write().unwrap();
                run_migrations(&mut conn).unwrap();
            }
            let person_repo = SqlitePersonRepository::arc_new(db_pool.clone());
            let agent_repo = SqliteAgentRepository::arc_new(db_pool.clone());
            let document_repo = SqliteDocumentRepository::arc_new(db_pool.clone());
            let household_repo = SqliteHouseholdRepository::arc_new(db_pool.clone());
            let service = Service::new(agent_repo, document_repo, household_repo, person_repo);

            prepare_and_run(
                "scrape",
                trace,
                verbose,
                ancestors_cli::shared::STANDARD_RANGE,
                move |_progress, out, err| {
                    let mut fetched: HashMap<String, String> = HashMap::new();
                    let res = gen_scraper::scrape(service, out, err, vec![url], &mut fetched);
                    let fetched_str = serde_json::to_string(&fetched).unwrap();
                    fs::write("fetched.json", fetched_str).unwrap();
                    res
                },
            )
        }?,
    }
    Ok(())
}
