use ancestors_cli::{cli::flags, shared::pretty::prepare_and_run};
use gen_repo_sqlite::SqlitePersonRepository;

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
            let repo = SqlitePersonRepository::arc_new("./data/ancestors.db3");
            prepare_and_run(
                "scrape",
                trace,
                verbose,
                ancestors_cli::shared::STANDARD_RANGE,
                move |_progress, out, err| gen_scraper::scrape(repo, out, err, vec![url]),
            )
        }?,
    }
    Ok(())
}
