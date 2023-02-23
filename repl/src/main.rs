use ancestors::application::use_cases::{AddPerson, AddingPerson};
use ancestors::infrastructure::MemGedcomxPersonRepo;
use clap::{crate_name, crate_version, value_parser, Arg, Command};
use repl::AppContext;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;

fn main() -> eyre::Result<()> {
    env_logger::init();

    // Starting Rustyline with a default configuration
    // let config = get_config();

    // Getting a new Rustyline Helper
    // let helper = REPLHelper::default();

    // Initiatlizing Rustyline Editor with set config and setting helper
    // let mut repl = Editor::with_config(config);
    let mut repl: Editor<()> = Editor::new()?;
    // repl.set_helper(Some(helper));

    // This method loads history file into memory
    // If it doesn't exist, creates one
    // TODO: Check history file size and if too big, clean it.
    if repl.load_history("data/history").is_err() {
        println!("No previous history.");
    }

    // Friendly intro message for the user
    println!(
        "{} - {}\n{}{}{}{}",
        crate_name!(),
        crate_version!(),
        "Enter .exit to quit.\n",
        "Enter .help for usage hints.\n",
        "Connected to a transient in-memory database.\n",
        "Use '.open FILENAME' to reopen on a persistent database."
    );

    // let mut db = Database::new("tempdb".to_string());
    // let db = Arc::new(RwLock::new(GedcomX::new()));

    let mut ctx = AppContext::default();

    loop {
        let p = format!(">> ");
        // repl.helper_mut().expect("No helper found").colored_prompt =
        //     format!("\x1b[1;32m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                match respond(&command, &mut ctx) {
                    Ok(quit) => {
                        if quit {
                            break;
                        }
                    }
                    Err(err) => {
                        println!("{err:?}");
                    }
                }
                // Parsing user's input and returning and enum of repl::CommandType
                // match get_command_type(&command.trim().to_owned()) {
                //     CommandType::SQLCommand(_cmd) => {
                //         // process_command takes care of tokenizing, parsing and executing
                //         // the SQL Statement and returning a Result<String, SQLRiteError>
                //         let _ = match process_command(&command, &mut db) {
                //             Ok(response) => println!("{}", response),
                //             Err(err) => eprintln!("An error occured: {}", err),
                //         };
                //     }
                //     CommandType::MetaCommand(cmd) => {
                //         // handle_meta_command parses and executes the MetaCommand
                //         // and returns a Result<String, SQLRiteError>
                //         let _ = match handle_meta_command(cmd, &mut repl) {
                //             Ok(response) => println!("{}", response),
                //             Err(err) => eprintln!("An error occured: {}", err),
                //         };
                //     }
                // }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("An error occured: {:?}", err);
                break;
            }
        }
    }
    repl.append_history("data/history").unwrap();

    Ok(())
}

fn respond(line: &str, ctx: &mut AppContext) -> eyre::Result<bool> {
    let args = shlex::split(line).unwrap();
    println!("args = {args:?}");
    let matches = cli().try_get_matches_from(args)?;

    match matches.subcommand() {
        Some(("exit", _matches)) => return Ok(true),
        Some(("load", matches)) => {
            let path = matches.get_one::<PathBuf>("file").expect("required");
            ctx.load(path)?;
        }
        Some(("save-as", matches)) => {
            let path = matches.get_one::<PathBuf>("file").expect("required");
            ctx.save_as(path)?;
        }
        Some(("person", submatches)) => {
            println!("person");
            match submatches.subcommand() {
                Some(("list", _matches)) => {
                    println!("list persons");
                    println!("persons = {:#?}", ctx.db().read().unwrap().persons());
                }
                Some(("add", add_matches)) => {
                    let name: &String = add_matches.get_one("name").unwrap();
                    // let person = Person::new("id").name(name);
                    // println!("{person:?}");
                    let cmd = AddPerson {
                        name: Some(name.clone()),
                        ..Default::default()
                    };
                    let repo = MemGedcomxPersonRepo::arc_new(ctx.db().clone());
                    let uc = AddingPerson::new(repo);
                    uc.execute(&cmd).unwrap();
                }
                Some(("edit", edit_matches)) => {
                    let id: Option<&String> = edit_matches.get_one("id");
                    let id: String = match id {
                        Some(id) => id.clone(),
                        None => {
                            for (i, person) in ctx.db().read().unwrap().persons().iter().enumerate()
                            {
                                let name: &str = if person.names().is_empty() {
                                    ""
                                } else {
                                    if person.names()[0].name_forms().is_empty() {
                                        ""
                                    } else {
                                        person.names()[0].name_forms()[0].get_full_text()
                                    }
                                };
                                println!("{}: {}", i, name);
                            }
                            let mut choice = None;
                            loop {}
                            "name".into()
                        }
                    };
                    println!("edit person {:?}", id);
                }
                _ => todo!("handle other"),
            }
        }
        Some((name, _matches)) => unimplemented!("{}", name),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("load")
                .about("read from file")
                .help_template(APPLET_TEMPLATE)
                .arg(
                    Arg::new("file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("save-as")
                .about("write to file")
                .help_template(APPLET_TEMPLATE)
                .arg(
                    Arg::new("file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("exit")
                .alias("quit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommands(applet_commands())
}

fn applet_commands() -> [Command; 1] {
    [person_commands()]
}

fn person_commands() -> Command {
    Command::new("person")
        .about("all about persons")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("add").arg(Arg::new("name").required(true)))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("edit").arg(Arg::new("id")))
}
