use crate::commands::categories::prompt_create_categories;
use clap::{arg, command, Command};
use std::path::Path;

use crate::{data_storage, models::Project};

pub mod categories;
pub mod list_items;
pub mod tasks;
pub mod user;

pub fn parse() -> Result<(), inquire::error::InquireError> {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initializes new project"))
        .subcommand(Command::new("status").about("Prints status of the project"))
        .subcommand(Command::new("print").about("Prints options for the project"))
        .subcommand(Command::new("categories").about("Alter categoris of the project"))
        .subcommand(Command::new("tasks").about("Alter tasks of the project"))
        .subcommand(Command::new("users").about("Alter users of project"))
        .subcommand(
            Command::new("print-task")
                .about("Prints one task")
                .arg(arg!(["ID"])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init()?,
        Some(("status", _)) => {
            let p = data_storage::load_project()?;
            p.print_status()
        }
        Some(("categories", _)) => categories::prompt_categories()?,
        Some(("tasks", _)) => tasks::prompt_tasks()?,
        Some(("users", _)) => user::prompt_users()?,
        Some(("print-task", args)) => {
            let id: u64 = args.get_one::<String>("ID").unwrap().parse().unwrap();
            let p = data_storage::load_project()?;
            p.print_single_task(id);
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
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

    println!("Found git repo at {}", git_location.display());

    if Path::new(".crabd").exists() {
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
