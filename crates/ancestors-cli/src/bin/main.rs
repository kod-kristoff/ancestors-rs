use ancestors_cli::{cli::flags, shared::pretty::prepare_and_run};

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}
fn try_main() -> miette::Result<()> {
    let flags = flags::AncestorsCli::from_env_or_exit();

    let verbose = flags.verbose;
    let trace = flags.trace;

    let cmd = flags.subcommand;
    match cmd {
        flags::AncestorsCliCmd::Scrape(flags::Scrape { url }) => {
            prepare_and_run(
                "scrape",
                trace,
                verbose,
                None,
                move |_progress, out, err| gen_scraper::scrape(out, err, vec![url]),
            )
        }?,
    }
    Ok(())
}
