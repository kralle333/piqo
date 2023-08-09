use clap::ArgMatches;
use inquire::{MultiSelect, Select};

use crate::{data_storage, models::Project};

use super::list_items::CategoryItem;

pub(crate) fn prompt_categories(
    category_matches: &ArgMatches,
) -> Result<(), inquire::error::InquireError> {
    let mut p = data_storage::load_project().unwrap();
    match category_matches.subcommand() {
        Some(("add", _)) => prompt_create_categories(&mut p).unwrap(),
        Some(("remove", _)) => prompt_remove_categories(&mut p).unwrap(),
        Some(("edit", _)) => prompt_edit_category(&mut p).unwrap(),
        Some(("list", _)) => p.print_categories(),
        Some(("print", _)) => {
            let selected = Select::new("Select category:", get_categories_list(&p)).prompt()?;

            p.print_category(selected.id);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    data_storage::store_project(&p)?;
    Ok(())
}

pub(crate) fn prompt_create_category(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let status_name = inquire::Text::new("Category name").prompt()?;
    p.add_category(status_name.as_str());
    Ok(())
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
        prompt_create_category(p)?
    }
}

pub(crate) fn prompt_remove_categories(
    p: &mut Project,
) -> Result<(), inquire::error::InquireError> {
    let categories = get_categories_list(p);
    let categories_to_remove =
        MultiSelect::new("Select categories to remove", categories).prompt()?;
    for i in categories_to_remove {
        p.remove_category(i.id);
    }
    Ok(())
}

pub(crate) fn prompt_edit_category(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let categories = get_categories_list(p);
    let category_to_edit = inquire::Select::new("Select category to edit", categories).prompt()?;
    let new_name = inquire::Text::new("New name").prompt()?;
    p.edit_category(category_to_edit.id, new_name.as_str());
    Ok(())
}

pub(crate) fn get_categories_list(p: &Project) -> Vec<CategoryItem> {
    p.categories
        .iter()
        .map(|t| -> CategoryItem {
            CategoryItem {
                id: t.id,
                name: t.name.to_owned(),
            }
        })
        .collect::<Vec<CategoryItem>>()
}
