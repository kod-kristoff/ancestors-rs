use ancestors_r3bl::app::{run_tui_app, AppError};

#[tokio::main]
async fn main() -> error_stack::Result<(), AppError> {
    run_tui_app().await
}
