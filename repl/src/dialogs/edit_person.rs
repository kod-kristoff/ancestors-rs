use ancestors_core::component::person::application::service::person_service::EditPerson;
use clap::Command;

use crate::app::AppContext;

pub fn edit_person(id: String, ctx: &mut AppContext) -> eyre::Result<Option<EditPerson>> {
    let matches = edit_person_commands().try_get_matches()?;

    match matches.subcommand() {
        Some((name, _matches)) => unimplemented!("{}", name),
        None => unreachable!("subcommand required"),
    }
}

pub fn edit_person_commands() -> Command {
    Command::new("edit_person")
        // .multicall(true)
        .about("edit a person")
        .arg_required_else_help(true)
        .subcommand_required(true)
        // .subcommand(Command::new("add").arg(Arg::new("name").required(true)))
        .subcommand(Command::new("show"))
    // .subcommand(Command::new("edit").arg(Arg::new("id")))
}
