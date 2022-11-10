// use rustyline::config::OutputStreamType;
use rustyline::Config;

use crate::person::PersonCommand;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    PersonCommand(PersonCommand),
}
pub fn get_command_type(command: &String) -> CommandType {
    if command.starts_with("person") {
        CommandType::PersonCommand(PersonCommand::new(command.into()))
    } else {
        todo!("handle unsupported command")
    }
}
pub fn get_config() -> Config {
    Config::builder()
        // .output_stream(OutputStreamType::Stdout)
        .build()
}
