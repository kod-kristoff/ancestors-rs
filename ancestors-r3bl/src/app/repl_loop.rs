use std::fmt;

use error_stack::ResultExt;

use crate::r3bl_goodies::{print_header, readline_with_prompt, style_error};

pub async fn run_tui_app() -> error_stack::Result<(), AppError> {
    println!("run_tui_app");
    repl_loop(create_store().await).await?;
    Ok(())
}

#[derive(Debug)]
pub struct AppError;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("app error")
    }
}

impl error_stack::Context for AppError {}

async fn create_store() -> &'static str {
    println!("create_store");
    "store"
}

pub type AppResult<T> = error_stack::Result<T, AppError>;

pub async fn repl_loop(store: &str) -> AppResult<()> {
    println!("repl_loop({})", store);
    print_header("Starting repl");

    loop {
        let user_input = readline_with_prompt("anc >").change_context(AppError)?;
        match user_input.as_str() {
            "quit" | "exit" => break,
            _ => println!("{}", style_error("unknown command")),
        }
    }
    Ok(())
}
