use clap::ArgMatches;
use inquire::ui::{Color, RenderConfig, Styled};
use inquire::{MultiSelect, Select};

use super::super::data_storage;
use super::super::models::Project;

use super::categories::get_categories_list;
use super::list_items::TaskItem;
use super::{categories, users};

pub(crate) fn prompt_tasks(task_matches: &ArgMatches) -> Result<(), inquire::error::InquireError> {
    let mut p = data_storage::load_project()?;
    match task_matches.subcommand() {
        Some(("add", _)) => prompt_create_tasks(&mut p)?,
        Some(("archieve", _)) => prompt_archieve_tasks(&mut p)?,
        Some(("assign", _)) => users::prompt_assign_users(&mut p)?,
        Some(("unassign", _)) => users::prompt_unassign_users(&mut p)?,
        Some(("move", _)) => prompt_move_tasks(&mut p)?,
        Some(("edit", _)) => prompt_edit_task(&mut p)?,
        Some(("list", _)) => p.print_tasks(),
        Some(("print", _)) => {
            let selected_task = Select::new("Select task:", get_tasks_list(&p))
                .prompt()
                .unwrap();
            p.print_single_task(selected_task.id);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    data_storage::store_project(&p)?;
    Ok(())
}

pub(crate) fn get_tasks_list(p: &Project) -> Vec<TaskItem> {
    p.tasks
        .iter()
        .map(|t| TaskItem {
            id: t.id,
            name: t.name.to_owned(),
        })
        .collect::<Vec<TaskItem>>()
}

fn prompt_move_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_tasks =
        MultiSelect::new("Select tasks to move:", get_categories_list(p)).prompt()?;

    let selected_category =
        Select::new("Select category:", categories::get_categories_list(p)).prompt()?;

    for task in selected_tasks {
        p.move_task(task.id, selected_category.id);
    }

    Ok(())
}

fn prompt_edit_task(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_task = Select::new("Select task:", get_categories_list(p))
        .prompt()
        .unwrap();
    let selected_field = Select::new("Select field:", vec!["Name", "Description"])
        .prompt()
        .unwrap();
    match selected_field {
        "Name" => {
            let new_name = inquire::Text::new("New name:").prompt()?;
            p.edit_task_name(selected_task.id, new_name);
        }
        "Description" => {
            let new_description = inquire::Editor::new("New description:").prompt()?;
            p.edit_task_description(selected_task.id, new_description);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    Ok(())
}

fn prompt_archieve_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_tasks =
        MultiSelect::new("Select tasks to archieve:", get_tasks_list(p)).prompt()?;

    for task in selected_tasks {
        p.archieve_task(task.id);
    }

    Ok(())
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

pub(crate) fn prompt_create_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    prompt_create_task(p)?;
    loop {
        let create_more = inquire::Select::new("Create more tasks?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        prompt_create_task(p)?;
    }
}
fn description_render_config() -> inquire::ui::RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}
