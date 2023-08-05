use clap::{arg, command, Command};
use gix::gix_location;
use inquire::ui::Color;
use inquire::ui::RenderConfig;
use inquire::ui::Styled;
use std::path::Path;

use crate::{data_storage, models::Project};

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
        Some(("tasks", _)) => {
            let taskMatches = command!()
                .propagate_version(true)
                .subcommand_required(true)
                .arg_required_else_help(true)
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
                )
                .get_matches();
            let p = data_storage::load_project()?;
            match taskMatches.subcommand() {
                Some(("add", _)) => prompt_create_tasks(&mut p)?,
                Some(("remove", _)) => p.remove_task()?,
                Some(("assign", _)) => p.assign_task()?,
                Some(("unassign", _)) => p.unassign_task()?,
                Some(("move", _)) => p.move_task()?,
                Some(("edit", _)) => p.edit_task()?,
                Some(("print", _)) => p.print_tasks(),
                Some(("print-task", args)) => {
                    let id: u64 = args.get_one::<String>("ID").unwrap().parse().unwrap();
                    p.print_single_task(id);
                }
                _ => unreachable!(
                    "Exhausted list of subcommands and subcommand_required prevents `None`"
                ),
            };
            p.print_tasks()
        }
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
    let git_location = gix_location()?;

    if git_location.is_none() {
        println!("Not a git repository");
        return Ok(());
    }
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
        prompt_create_tasks(&mut p)?;
    }

    data_storage::store_project(&p)?;
    Ok(())
}

fn prompt_create_categories(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    loop {
        let create_more =
            inquire::Select::new("Create more categories?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        let status_name = inquire::Text::new("Category name").prompt()?;
        p.add_category(status_name.as_str());
    }
}

fn prompt_create_task(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let name = inquire::Text::new("Name:").prompt()?;
    let description = inquire::Editor::new("Description:")
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else if char_count <= 20 {
                submission.into()
            } else {
                let mut substr: String = submission.chars().take(17).collect();
                substr.push_str("...");
                substr
            }
        })
        .with_render_config(description_render_config())
        .prompt()?;

    p.add_task(name, description);
    Ok(())
}

fn description_render_config() -> inquire::ui::RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}

fn prompt_create_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    prompt_create_task(p)?;
    loop {
        let create_more = inquire::Select::new("Create more tasks?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        prompt_create_task(p)?;
    }
}
