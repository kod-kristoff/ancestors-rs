use ancestors_kernel::context::AppContext;
use ancestors_tui::app::AppComponent;
use ancestors_tui::config::Config;
use ancestors_tui::event::EventHandler;
use ancestors_tui::run_app;
use ancestors_tui::tui::Tui;
use eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Create an application.
    let config = Config::new()?;
    let ctx = AppContext::default();
    let mut app = AppComponent::new(config, ctx);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let res = run_app(&mut tui, &mut app);

    // Exit the user interface.
    tui.exit()?;
    res?;
    Ok(())
}
// // use std::{fs, io};

// // use env_logger::{Env, Target};

// // fn main() {
// //     let log_file = fs::File::create("logs/ancestors.log").expect("creating log file");
// //     let log_writer = Box::new(io::BufWriter::new(log_file));
// //     env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
// //         // .format_timestamp(None)
// //         .target(Target::Pipe(log_writer))
// //         .init();

// //     if let Err(err) = ancestors_tui::try_main() {
// //         println!("Error occured: {:#?}", err);
// //         std::process::exit(1);
// //     }
// // }
