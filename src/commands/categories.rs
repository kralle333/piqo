use clap::{arg, command, Command};
use inquire::MultiSelect;

use crate::{data_storage, models::Project};

use super::list_items::CategoryItem;

pub(crate) fn prompt_categories() -> Result<(), inquire::error::InquireError> {
    let category_matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("add").about("Adds new category"))
        .subcommand(Command::new("remove").about("Removes category"))
        .subcommand(Command::new("edit").about("Edits category"))
        .subcommand(Command::new("print").about("Prints categories"))
        .subcommand(
            Command::new("print-category")
                .about("Prints one category")
                .arg(arg!(["ID"])),
        )
        .get_matches();

    let mut p = data_storage::load_project().unwrap();
    match category_matches.subcommand() {
        Some(("add", _)) => prompt_create_categories(&mut p).unwrap(),
        Some(("remove", _)) => prompt_remove_categories(&mut p).unwrap(),
        // Some(("edit", _)) => prompt_edit_categories(&mut p).unwrap(),
        // Some(("print", _)) => p.print_categories(),
        Some(("print-category", args)) => {
            let id: u64 = args.get_one::<String>("ID").unwrap().parse().unwrap();
            print_single_category(&p, id);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    Ok(())
}

fn print_single_category(p: &Project, id: u64) {
    let category = p.get_category(id).unwrap();
    println!("{}", category.name());
}

pub(crate) fn prompt_create_categories(
    p: &mut Project,
) -> Result<(), inquire::error::InquireError> {
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

pub(crate) fn prompt_remove_categories(
    p: &mut Project,
) -> Result<(), inquire::error::InquireError> {
    let categories = get_mod_list(p);
    let categories_to_remove =
        MultiSelect::new("Select categories to remove", categories).prompt()?;
    for i in categories_to_remove {
        p.remove_category(i.id);
    }
    Ok(())
}

pub(crate) fn get_mod_list(p: &Project) -> Vec<CategoryItem> {
    p.categories()
        .iter()
        .map(|t| CategoryItem {
            id: *t.id(),
            name: t.name().to_owned(),
        })
        .collect::<Vec<CategoryItem>>()
}
