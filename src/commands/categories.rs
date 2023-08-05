use clap::ArgMatches;
use inquire::MultiSelect;

use crate::{data_storage, models::Project};

use super::list_items::CategoryItem;

pub(crate) fn prompt_categories(
    category_matches: &ArgMatches,
) -> Result<(), inquire::error::InquireError> {
    let mut p = data_storage::load_project().unwrap();
    match category_matches.subcommand() {
        Some(("add", _)) => prompt_create_categories(&mut p).unwrap(),
        Some(("remove", _)) => prompt_remove_categories(&mut p).unwrap(),
        // Some(("edit", _)) => prompt_edit_categories(&mut p).unwrap(),
        // Some(("print", _)) => p.print_categories(),
        Some(("print", args)) => {
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
