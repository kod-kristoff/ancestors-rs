// use rustyline::config::OutputStreamType;
use rustyline::Config;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    PersonCommand(PersonCommand),
}
pub fn get_command_type(command: &String) -> CommandType {}
pub fn get_config() -> Config {
    Config::builder()
        // .output_stream(OutputStreamType::Stdout)
        .build()
}
