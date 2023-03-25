use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::{fs::OpenOptions, path::Path};

/// Setup logging
pub fn setup_logging(log_level: LevelFilter, path: &Path) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(false)
        .truncate(true)
        .write(true)
        .open(path)?;

    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
    WriteLogger::init(log_level, config, file)?;
    Ok(())
}
