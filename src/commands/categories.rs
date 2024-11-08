use crate::{data_storage, models::Project};
use clap::ArgMatches;
use inquire::{MultiSelect, Select};

use super::list_items::CategoryItem;

pub(crate) fn prompt_categories(
    category_matches: &ArgMatches,
) -> Result<(), inquire::error::InquireError> {
    let mut p = data_storage::load_project()?;
    match category_matches.subcommand() {
        Some(("add", _)) => prompt_create_categories(&mut p)?,
        Some(("remove", _)) => prompt_remove_categories(&mut p)?,
        Some(("edit", _)) => prompt_edit_category(&mut p)?,
        Some(("list", _)) => p.print_categories(),
        Some(("print", _)) => {
            let selected =
                Select::new("Select category:", get_categories_list(&p, false)).prompt()?;

            p.print_category(selected.id);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    data_storage::store_project(&p)?;
    Ok(())
}

pub(crate) fn prompt_create_category(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let status_name = inquire::Text::new("Category name").prompt()?;
    if p.categories.iter().any(|c| c.name == status_name) {
        println!("Category with this name already exists");
        return Err(inquire::error::InquireError::OperationCanceled);
    }
    p.add_category(status_name.as_str());
    Ok(())
}

pub(crate) fn prompt_create_categories(
    p: &mut Project,
) -> Result<(), inquire::error::InquireError> {
    prompt_create_category(p)?;
    loop {
        let create_more =
            Select::new("Create more categories?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        prompt_create_category(p)?
    }
}

pub(crate) fn prompt_remove_categories(
    p: &mut Project,
) -> Result<(), inquire::error::InquireError> {
    let categories = get_categories_list(p, true);

    let not_deletable: Vec<&CategoryItem> = categories.iter().filter(|c| c.not_deletable).collect();
    if !not_deletable.is_empty() {
        println!("The following categories are not deletable as they contain tasks:");
        for category in &not_deletable {
            if category.not_deletable {
                println!("{}", category.name);
            }
        }
        println!();
    }

    let categories_to_remove = MultiSelect::new(
        "Select categories to remove",
        categories.iter().filter(|c| !c.not_deletable).collect(),
    )
        .prompt()?;

    for i in categories_to_remove {
        p.remove_category(i.id);
    }
    Ok(())
}

pub(crate) fn prompt_edit_category(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let categories = get_categories_list(p, false);
    let category_to_edit = Select::new("Select category to edit", categories).prompt()?;
    let new_name = inquire::Text::new("New name").prompt()?;
    p.edit_category(category_to_edit.id, new_name.as_str());
    Ok(())
}

pub(crate) fn get_categories_list(p: &Project, check_deletable: bool) -> Vec<CategoryItem> {
    p.categories
        .iter()
        .map(|t| -> CategoryItem {
            CategoryItem {
                id: t.id,
                name: t.name.to_owned(),
                not_deletable: check_deletable && p.tasks.iter().any(|task| task.category == t.id),
            }
        })
        .collect::<Vec<CategoryItem>>()
}
