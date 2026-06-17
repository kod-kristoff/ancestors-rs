use cosmic::{app::Settings, iced::Size};
use tracing_log::log;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::{App, Flags};

pub mod app;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_format = tracing_subscriber::fmt::format()
        .pretty()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_thread_names(true);

    let log_layer = tracing_subscriber::fmt::Layer::default()
        .with_writer(std::io::stderr)
        .event_format(log_format);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy()
                .add_directive("ancestors_gui=info".parse()?),
        )
        .with(log_layer)
        .init();

    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false)
        .default_icon_theme("Pop")
        .default_text_size(16.0)
        .scale_factor(1.0)
        .size(Size::new(1024., 768.));

    let flags = Flags {};
    log::info!("Starting app");
    cosmic::app::run::<App>(settings, flags)?;

    Ok(())
}
