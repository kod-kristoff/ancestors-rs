use std::{fs, io};

use env_logger::{Env, Target};

fn main() {
    let log_file = fs::File::create("ancestors.log").expect("creating log file");
    let log_writer = Box::new(io::BufWriter::new(log_file));
    env_logger::Builder::from_env(Env::default().default_filter_or("trace,rustyline=warn,mio=error"))
        .format_timestamp(None)
        .target(Target::Pipe(log_writer))
        .init();

    if let Err(err) = ancestors_tui::try_main() {
        println!("Error occured: {:#?}", err);
        std::process::exit(1);
    }
}
