use chrono::{NaiveDateTime, NaiveTime, TimeZone, Timelike};
use clap::ArgMatches;

use inquire::validator::Validation;
use inquire::{CustomType, DateSelect, MultiSelect, Select};

use super::super::data_storage;
use super::super::models::Project;

use super::list_items::{DueTime, TaskItem};
use super::{categories, users};

pub(crate) fn prompt_tasks(task_matches: &ArgMatches) -> Result<(), inquire::error::InquireError> {
    let mut p = data_storage::load_project()?;
    match task_matches.subcommand() {
        Some(("add", _)) => prompt_create_tasks(&mut p)?,
        Some(("archive", _)) => prompt_archive_tasks(&mut p)?,
        Some(("assign", _)) => users::prompt_assign_users(&mut p)?,
        Some(("unassign", _)) => users::prompt_unassign_users(&mut p)?,
        Some(("move", _)) => prompt_move_tasks(&mut p)?,
        Some(("edit", _)) => prompt_edit_task(&mut p)?,
        Some(("list", _)) => p.print_tasks(),
        Some(("remove", _)) => prompt_remove_tasks(&mut p)?,
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

fn prompt_remove_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_tasks = MultiSelect::new("Select tasks to remove:", get_tasks_list(p)).prompt()?;

    for task in selected_tasks {
        let assigned_users = p.get_assigned_users(task.id);
        let remove =
            inquire::Confirm::new(&format!("Confirm deletion of task {}", task.name)).prompt()?;

        if !remove {
            continue;
        }

        for user in assigned_users {
            p.unassign_task(user.id, task.id);
        }
        p.remove_task(task.id);
    }

    Ok(())
}

pub(crate) fn get_tasks_list(p: &Project) -> Vec<TaskItem> {
    p.tasks
        .iter()
        .map(|t| TaskItem {
            id: t.id,
            name: t.name.to_owned(),
            category: None,
        })
        .collect::<Vec<TaskItem>>()
}
pub(crate) fn get_tasks_list_with_categories(p: &Project) -> Vec<TaskItem> {
    p.tasks
        .iter()
        .map(|t| TaskItem {
            id: t.id,
            name: t.name.to_owned(),
            category: p.get_category_name(t.category),
        })
        .collect::<Vec<TaskItem>>()
}

fn prompt_move_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_tasks =
        MultiSelect::new("Select tasks to move:", get_tasks_list_with_categories(p)).prompt()?;

    let categories = categories::get_categories_list(p, false);

    let selected_category = Select::new("Select category:", categories).prompt()?;

    for task in selected_tasks {
        p.move_task(task.id, selected_category.id);
    }

    Ok(())
}

fn prompt_get_due_time() -> Result<i64, inquire::error::InquireError> {
    let custom_date = DateSelect::new("Due date:")
        .with_help_message("When is the task due?")
        .prompt()?;

    let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let naive_date_time = NaiveDateTime::new(custom_date, naive_time);
    let due_date_day = chrono::Local.from_local_datetime(&naive_date_time).unwrap();

    let due_date_times = vec![DueTime::Noon, DueTime::Midnight, DueTime::Custom];

    let due_time = Select::new("Due time:", due_date_times)
        .with_help_message("Select a due time")
        .prompt()?;

    let (due_time_hour, due_time_minute) = match due_time {
        DueTime::Noon => (12, 0),
        DueTime::Midnight => (23, 59),
        DueTime::Custom => {
            let hour = CustomType::<u32>::new("due time hour:")
                .with_help_message("select a due time hour")
                .with_error_message("Please type a valid hour")
                .with_validator(|h: &u32| {
                    if *h <= 23 {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("invalid hour".into()))
                    }
                })
                .prompt()?;

            let minute = CustomType::<u32>::new("due time minutes:")
                .with_help_message("select a due time minute")
                .with_error_message("Please type a valid minute")
                .with_validator(|m: &u32| {
                    if *m <= 59 {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("invalid minute".into()))
                    }
                })
                .prompt()?;

            (hour, minute)
        }
    };

    let unix_time = due_date_day
        .with_hour(due_time_hour)
        .unwrap()
        .with_minute(due_time_minute)
        .unwrap();

    let unix_time_utc = unix_time.naive_utc().timestamp();
    Ok(unix_time_utc)
}

fn prompt_create_checklist_item(
    p: &mut Project,
    task_id: u64,
) -> Result<(), inquire::error::InquireError> {
    let name = inquire::Text::new("Checklist item name:")
        .with_help_message("What is the name of the checklist item?")
        .prompt()?;

    p.add_checklist_item(task_id, name);
    while inquire::Confirm::new("Add another checklist item?").prompt()? {
        let name = inquire::Text::new("Checklist item name:")
            .with_help_message("What is the name of the checklist item?")
            .prompt()?;
        p.add_checklist_item(task_id, name);
    }
    Ok(())
}

fn prompt_edit_task(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_task = Select::new("Select task:", get_tasks_list(p))
        .prompt()
        .unwrap();

    let mut fields = vec!["Name", "Description", "Checklist", "Set due date"];

    let task_due_time = p.get_task_due_time(selected_task.id);
    if task_due_time.is_some() {
        fields.push("Clear due date");
    }

    let selected_field = Select::new("Select field:", fields).prompt().unwrap();
    match selected_field {
        "Name" => {
            let new_name = inquire::Text::new("New name:").prompt()?;
            p.edit_task_name(selected_task.id, new_name);
        }
        "Description" => {
            let new_description = inquire::Editor::new("New description:")
                .with_predefined_text(p.get_task_description(selected_task.id))
                .prompt()?;
            p.edit_task_description(selected_task.id, new_description);
        }
        "Checklist" => {
            let checklist_option = Select::new(
                "Select checklist item:",
                vec!["Add checklist item", "Remove checklist item"],
            )
            .prompt()?;
            match checklist_option {
                "Add checklist item" => prompt_create_checklist_item(p, selected_task.id)?,
                "Remove checklist item" => {
                    let checklist_items = p.get_task_checklist(selected_task.id);
                    let selected_checklist_item =
                        Select::new("Select checklist item:", checklist_items).prompt()?;
                    p.remove_checklist_item(selected_task.id, selected_checklist_item.index);
                }
                _ => unreachable!(
                    "Exhausted list of subcommands and subcommand_required prevents `None`"
                ),
            }
        }

        "Set due date" => {
            let due_date = prompt_get_due_time()?;
            p.set_task_due_date(selected_task.id, due_date);
        }
        "Clear due date" => {
            let due_date = NaiveDateTime::from_timestamp_opt(task_due_time.unwrap(), 0)
                .unwrap()
                .to_string();
            let clear =
                inquire::Confirm::new(&format!("Clear due date ({})?", due_date)).prompt()?;
            if clear {
                p.clear_task_due_date(selected_task.id);
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    Ok(())
}

fn prompt_archive_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let selected_tasks =
        MultiSelect::new("Select tasks to archive:", get_tasks_list(p)).prompt()?;

    for task in selected_tasks {
        p.archive_task(task.id);
    }

    Ok(())
}

fn prompt_create_task(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    let name = inquire::Text::new("Name:").prompt()?;
    let task_id = p.add_task(name);

    users::prompt_select_user_to_assign(p, task_id)
}

pub(crate) fn prompt_create_tasks(p: &mut Project) -> Result<(), inquire::error::InquireError> {
    //TODO: use inquire::Confirm instead
    prompt_create_task(p)?;
    loop {
        let create_more = inquire::Select::new("Create more tasks?", vec!["Yes", "No"]).prompt()?;
        if create_more == "No" {
            return Ok(());
        }
        prompt_create_task(p)?;
    }
}
