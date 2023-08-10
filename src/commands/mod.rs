use std::env;

use crate::commands::categories::prompt_create_categories;
use clap::{command, Arg, ArgAction, Command};
use owo_colors::OwoColorize;

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
        .subcommand(
            Command::new("list").about("Lists project tasks").arg(
                Arg::new("json")
                    .long("json")
                    .action(ArgAction::SetTrue)
                    .help("output in json format"),
            ),
        )
        .subcommand(Command::new("status").about("Prints status of project"))
        .subcommand(
            Command::new("categories")
                .arg_required_else_help(true)
                .about("Alter categoris of the project")
                .subcommand(Command::new("add").about("Adds new category"))
                .subcommand(Command::new("remove").about("Removes category"))
                .subcommand(Command::new("edit").about("Edits category"))
                .subcommand(Command::new("list").about("Prints categories")),
        ) // .subcommand(Command::new("print").about("Prints details of one category")),
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
                .subcommand(Command::new("list").about("Prints tasks"))
                .subcommand(Command::new("print").about("Prints details of one task")),
        )
        .subcommand(
            Command::new("users")
                .arg_required_else_help(true)
                .about("Alter users of project")
                .subcommand(Command::new("add").about("Adds new users"))
                .subcommand(Command::new("remove").about("Removes users"))
                .subcommand(Command::new("assign").about("Assign users to tasks"))
                .subcommand(Command::new("list").about("Lists users")), // .subcommand(Command::new("print")),
        );

    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("init", _)) => init()?,
        Some(("status", _)) => {
            let p = data_storage::load_project()?;
            p.print_status()
            // p.print_status()
        }
        Some(("list", sync_matches)) => {
            let p = data_storage::load_project()?;

            if sync_matches.get_flag("json") {
                p.print_tasks_json();
            } else {
                p.print_tasks();
            }
        }
        Some(("categories", sub_matches)) => categories::prompt_categories(sub_matches)?,
        Some(("tasks", sub_matches)) => tasks::prompt_tasks(sub_matches)?,
        Some(("users", sub_matches)) => users::prompt_users(sub_matches)?,
        _ => {
            println!("unknown command")
        }
    };

    Ok(())
}

fn init() -> Result<(), inquire::error::InquireError> {
    let crabd_path = data_storage::check_crabd_dir();

    println!("crabd_path: ");
    match crabd_path {
        data_storage::CrabdPath::FoundNotInit(crabd_path) => {
            println!("Initializing project at: {}", crabd_path.to_str().unwrap());
        }
        data_storage::CrabdPath::Found(crabd_path) => {
            println!("Found crabd dir at {}", crabd_path.to_str().unwrap());
            println!("Project already initialized");
            return Ok(());
        }
        data_storage::CrabdPath::NotFound(err) => {
            return Err(inquire::InquireError::Custom(err.into()));
        }
    }
    let initial_project_name = match env::current_dir() {
        Ok(path) => path
            .iter()
            .last()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
        Err(_) => "".to_string(),
    };
    // check if is git repo and if not, init
    let name = inquire::Text::new("Project name:")
        .with_initial_value(&initial_project_name)
        .prompt()?;

    let mut p = Project::new(name);

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

    let create_users = inquire::Select::new("Create initial users?", vec!["Yes", "No"]).prompt()?;
    if create_users == "Yes" {
        users::prompt_add_users(&mut p)?;
    }

    data_storage::store_project(&p)?;
    Ok(())
}
