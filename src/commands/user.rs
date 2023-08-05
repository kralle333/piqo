use clap::{arg, command, Command};
use inquire::{MultiSelect, Select};

use crate::{
    data_storage,
    models::{Project, User},
};

use super::{list_items::UserItem, tasks};

pub(crate) fn prompt_users() -> Result<(), inquire::error::InquireError> {
    let users_matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("add").about("Add a user"))
        .subcommand(Command::new("remove").about("Remove a user"))
        .subcommand(Command::new("edit").about("Edit a user"))
        .subcommand(Command::new("print").about("Print all users"))
        .subcommand(
            Command::new("print")
                .about("Prints one user")
                .arg(arg!(["ID"])),
        )
        .get_matches();

    let mut p = data_storage::load_project().unwrap();
    match users_matches.subcommand() {
        Some(("add", _)) => prompt_add_users(&mut p).unwrap(),
        Some(("remove", _)) => prompt_remove_users(&mut p).unwrap(),
        Some(("assign", _)) => prompt_assign_users(&mut p).unwrap(),
        Some(("list", _)) => prompt_list(&mut p),
        Some(("print-user", args)) => {
            let id: u64 = args.get_one::<String>("ID").unwrap().parse().unwrap();
            print_single_user(&p, id);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };

    Ok(())
}

fn prompt_list(p: &mut Project) {
    let users = p.get_users();
    for ele in users {
        match ele.git_email() {
            Some(email) => println!("{} | {} | {}", ele.id(), ele.name(), email),
            None => println!("{} | {} | {}", ele.id(), ele.name(), "No email"),
        }
    }
}

//TODO: Fix this
fn print_single_user(p: &Project, id: u64) {
    match p.get_user(id) {
        Some(user) => println!("{:?}", user),
        None => println!("User not found"),
    }
}

fn get_users_mod_list(p: &Project) -> Vec<User> {
    p.get_users()
        .iter()
        .map(|u| User {
            id: u.id(),
            name: u.name().to_string(),
            git_email: u.git_email.to_owned(),
        })
        .collect()
}

fn prompt_assign_users(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_task =
        Select::new("Select Task To Assign", tasks::get_tasks_mod_list(p)).prompt()?;

    let users_to_assign =
        MultiSelect::new("Select Users To Assign", get_users_mod_list(p)).prompt()?;

    for user in users_to_assign {
        p.assign_task(user.id(), selected_task.id);
    }
    Ok(())
}

fn prompt_add_users(p: &mut crate::models::Project) -> Result<(), inquire::error::InquireError> {
    let option_a = "Scrape Git Users";
    let option_b = "Add user manually";
    let selections = vec![option_a, option_b];

    let choice = Select::new("Choose:", selections).prompt()?;
    match choice {
        "Scrape Git Users" => {
            let output = std::process::Command::new("git")
                .arg("log")
                .arg("--format=\"%an | %aE\" | sort -u")
                .output()?;

            let list = String::from_utf8(output.stdout);

            let split_one = list
                .unwrap()
                .lines()
                .map(|l| UserItem {
                    name: l.split(" | ").collect::<Vec<&str>>()[0].to_string(),
                    git_email: l.split(" | ").collect::<Vec<&str>>()[1].to_string(),
                })
                .collect::<Vec<UserItem>>();

            let choices = MultiSelect::new("Select Users To Add", split_one).prompt()?;

            for ele in choices {
                p.add_user(&ele.name, &ele.git_email);
            }

            Ok(())
        }
        "Add user manually" => {
            prompt_create_users(p)?;
            Ok(())
        }
        _ => unreachable!("Exhausted list of options and arg_required_else_help prevents `None`"),
    }
}
fn prompt_remove_users(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let users_to_remove =
        MultiSelect::new("Select Users To Remove", get_users_mod_list(p)).prompt()?;

    for ele in users_to_remove {
        p.remove_user(&ele);
    }
    Ok(())
}

fn prompt_create_user(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let name = inquire::Text::new("Name:").prompt()?;
    let email = inquire::Text::new("Email:").prompt()?;

    p.add_user(&name, &email);
    Ok(())
}

pub(crate) fn prompt_create_users(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    prompt_create_user(p)?;
    loop {
        let create_more = inquire::Select::new("Create more users?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        prompt_create_user(p)?;
    }
}
