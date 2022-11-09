mod repl;
pub mod person;

use repl::get_config;
use rustyline::error::ReadlineError;

pub fn try_main() -> eyre::Result<()> {
    log::trace!("called try_main");
    let config = get_config();
    let mut repl = rustyline::Editor::<()>::with_config(config)?;
    loop {
        let readline = repl.readline(">> ");
        match readline {
            Ok(command) => {
                log::trace!("Command: {:?}", command);
                match get_com
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                log::error!("An error occured: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
