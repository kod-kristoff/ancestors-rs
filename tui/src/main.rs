use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace,rustyline=warn"))
        .format_timestamp(None)
        .init();
    if let Err(err) = ancestors_tui::try_main() {
        println!("Error occured: {:#?}", err);
        std::process::exit(1);
    }
}
