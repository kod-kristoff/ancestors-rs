mod app;
mod contrib;
mod serialization;
mod telemetry;
mod ui;

use anyhow::Context;
use log::LevelFilter;
use std::path::Path;

use app::Runtime;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    set_panic_hook();
    if let Err(err) = try_main() {
        log::error!("{:?}", err);
        let _ = ui::reset_terminal();
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let log_level = LevelFilter::Trace;

    telemetry::setup_logging(log_level, Path::new("logs/ancestors-realm.log"))
        .context("Failed to setup logging")?;

    log::info!("Starting ancestors-realm {} ...", APP_VERSION);

    Runtime::setup()
        .context("Failed to setup runtime")?
        .run()
        .context("Error when running")?;
    Ok(())
}

fn set_panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let _ = ui::reset_terminal();
        original_hook(info);
    }));
}
