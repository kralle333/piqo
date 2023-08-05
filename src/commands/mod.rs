use crate::commands::categories::prompt_create_categories;
use clap::{arg, command, Arg, Command};
use std::path::Path;

use crate::{data_storage, models::Project};

pub mod categories;
pub mod list_items;
pub mod tasks;
pub mod users;

pub fn parse() -> Result<(), inquire::error::InquireError> {
    let command = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initializes new project"))
        .subcommand(Command::new("status").about("Prints status of the project"))
        .subcommand(Command::new("print").about("Prints options for the project"))
        .subcommand(
            Command::new("categories")
                .arg_required_else_help(true)
                .about("Alter categoris of the project")
                .subcommand(Command::new("add").about("Adds new category"))
                .subcommand(Command::new("remove").about("Removes category"))
                .subcommand(Command::new("edit").about("Edits category"))
                .subcommand(Command::new("prin").about("Prints categories"))
                .subcommand(
                    Command::new("print-category")
                        .about("Prints one category")
                        .arg(arg!(["ID"])),
                ),
        )
        .subcommand(
            Command::new("tasks")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .about("Alter tasks of the project")
                .subcommand(Command::new("add").about("Adds new task"))
                .subcommand(Command::new("remove").about("Removes task"))
                .subcommand(Command::new("assign").about("Assigns task to user"))
                .subcommand(Command::new("unassign").about("Unassigns task from user"))
                .subcommand(Command::new("move").about("Moves tasks to another category"))
                .subcommand(Command::new("edit").about("Edits task"))
                .subcommand(Command::new("print").about("Prints tasks"))
                .subcommand(
                    Command::new("print-task")
                        .about("Prints one task")
                        .arg(arg!(["ID"])),
                ),
        )
        .subcommand(
            Command::new("users")
                .arg_required_else_help(true)
                .about("Alter users of project")
                .subcommand(Command::new("add").about("Adds new users"))
                .subcommand(Command::new("remove").about("Removes users"))
                .subcommand(Command::new("assign").about("Assign users to tasks"))
                .subcommand(Command::new("list").about("Lists Users"))
                .subcommand(
                    Command::new("print")
                        .about("Print a single user")
                        .arg(Arg::new("ID").required(true)),
                ),
        )
        .subcommand(
            Command::new("print-task")
                .about("Prints one task")
                .arg(arg!(["ID"])),
        );

    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("init", _)) => init()?,
        Some(("status", _)) => {
            let p = data_storage::load_project()?;
            p.print_status()
        }
        Some(("categories", sub_matches)) => categories::prompt_categories(sub_matches)?,
        Some(("tasks", sub_matches)) => tasks::prompt_tasks(sub_matches)?,
        Some(("users", sub_matches)) => users::prompt_users(sub_matches)?,
        Some(("print-task", args)) => {
            let id: u64 = args.get_one::<String>("ID").unwrap().parse().unwrap();
            let p = data_storage::load_project()?;
            p.print_single_task(id);
        }
        _ => {
            println!("unkown command")
        }
    };

    Ok(())
}

fn init() -> Result<(), inquire::error::InquireError> {
    let git_location = gix_discover::upwards(Path::new("."));

    let git_location = match git_location {
        Ok(git_path) => git_path.0,
        Err(err) => {
            println!("Git repo not found. Requirement for crabd to work: {}", err);
            return Ok(());
        }
    };

    let (er, _) = git_location.into_repository_and_work_tree_directories();

    let crabd_json_path = er.join(".crabd");

    if Path::new(crabd_json_path.to_str().unwrap()).exists() {
        println!("Project already initialized");
        return Ok(());
    }
    // check if is git repo and if not, init
    if !Path::new(".git").exists() {
        let create_file = inquire::Select::new(
            "No .git folder found, init project anyway?",
            vec!["Yes", "No"],
        )
        .prompt()?;
        if create_file == "No" {
            return Ok(());
        }
    }
    let name = inquire::Text::new("Project name").prompt()?;

    let mut p = Project::new(name);
    data_storage::store_project(&p)?;

    let create_categories =
        inquire::Select::new("Set initial categories", vec!["Default", "Custom"]).prompt()?;
    if create_categories == "Default" {
        p.add_default_category("Todo");
        p.add_category("In Progress");
        p.add_category("Done");
    } else {
        let default_status = inquire::Text::new("Default status Name").prompt()?;
        p.add_default_category(default_status.as_str());
        prompt_create_categories(&mut p)?;
    }

    let create_tasks = inquire::Select::new("Create initial tasks?", vec!["Yes", "No"]).prompt()?;
    if create_tasks == "Yes" {
        tasks::prompt_create_tasks(&mut p)?;
    }

    data_storage::store_project(&p)?;
    Ok(())
}
