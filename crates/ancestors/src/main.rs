use ancestors::app::AppView;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("ancestors=debug".parse().unwrap())
                .from_env()
                .unwrap(),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting global subscriber");

    tracing::info!("Starting ancestors.");

    let app = AppView::new();
    app.run();
}
