use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use std::collections::HashMap;

use crate::models::Project;
use crate::models::{Task, TaskJson, User};
use crate::utils;
use crate::utils::truncate as t;
use crate::utils::truncate_then_center_align as ct;

impl Project {
    pub(crate) fn print_tasks_detailed(&self) {
        for task in self.get_unarchived_tasks() {
            println!("{}", task.name.green());
            if !task.description.is_empty() {
                Self::print_description(&task.description);
            }
            println!(
                "Category: {}",
                self.get_category(task.category).unwrap().name
            );

            if !task.check_list.is_empty() {
                println!("Checklist:");
                for item in &task.check_list {
                    let line = match item.checked {
                        true => format!("- [x] {}", item),
                        false => format!("- [ ] {}", item),
                    };
                    println!("{}", line);
                }
            }
            if task.due_date_utc.is_some() {
                println!(
                    "Due date: {}",
                    DateTime::from_timestamp(task.due_date_utc.unwrap(), 0).unwrap()
                );
            }
            println!("Assigned to:");
            if task.assigned_to.is_empty() {
                println!("- None");
            } else {
                self.get_assigned_users(task.id).iter().for_each(|u| {
                    println!(
                        "- {} <{}>",
                        u.name,
                        u.git_email.to_owned().unwrap_or("no email".to_string())
                    );
                });
            }
            println!();
        }
    }

    pub(crate) fn print_tasks(&self) {
        let mut tasks = self.tasks.clone();
        tasks.sort_by(|a, b| a.category.cmp(&b.category));

        let mut category_names = HashMap::new();
        for category in &self.categories {
            category_names.insert(category.id, &category.name);
        }

        let mut user_names = HashMap::new();
        for user in &self.users {
            user_names.insert(&user.id, &user.name);
        }

        let has_any_with_due_date = tasks.iter().any(|t| t.due_date_utc.is_some());

        //TODO: This code is a nightmare.
        // Suggestion: built formatting string with percentages of how much space each field should take up
        let mut l = vec![36, 0, 12, 30];

        if !has_any_with_due_date {
            println!(
                "{:^a$}|{:^b$}|{:^c$}",
                "Name".bold(),
                "Category".bold(),
                "Assigned To".bold(),
                a = l[0],
                b = l[2],
                c = l[3],
            );
        } else {
            l[0] = 30;
            l[1] = 6;
            println!(
                "{:^a$}|{:^b$}|{:^c$}|{:^d$}",
                "Name".bold(),
                "Due".bold(),
                "Category".bold(),
                "Assigned To".bold(),
                a = l[0],
                b = l[1],
                c = l[2],
                d = l[3],
            );
        }

        println!("{}", "-".repeat(80));

        tasks.iter().for_each(|task| {
            let users = task
                .assigned_to
                .iter()
                .map(|i| self.get_user(*i).unwrap())
                .collect::<Vec<User>>();

            let assigned_to = match task.assigned_to.is_empty() {
                false => {
                    let total_space = l[3] - (users.len() - 1);
                    let space_per_name = total_space / users.len();
                    users
                        .iter()
                        .map(|f| t(user_names.get(&f.id).unwrap(), space_per_name))
                        .collect::<Vec<String>>()
                        .join(",")
                }

                true => "None".to_string(),
            };

            if has_any_with_due_date {
                print!("{:<a$}|", &t(&task.name, l[0]), a = l[0]);
                if task.due_date_utc.is_some() {
                    let seconds_till = task.due_date_utc.unwrap() - Utc::now().timestamp();
                    let (formatted_due_date, style) = &utils::display_due_date_time(seconds_till);
                    // Printing this way was the only way for me to get colors formatted correctly
                    print!("{:^b$}|", &formatted_due_date.style(*style), b = l[1]);
                } else {
                    print!("{:^b$}|", "", b = l[1]);
                }

                println!(
                    "{:^c$}|{:^d$}",
                    &self.get_category(task.category).unwrap().name.to_string(),
                    &t(&assigned_to, l[3]),
                    c = l[2],
                    d = l[3],
                );
                return;
            }
            println!(
                "{:<a$}|{:^b$}|{:^c$}",
                &t(&task.name, l[0]),
                // &t(&task.description, l[1]),
                &t(
                    &self.get_category(task.category).unwrap().name.to_string(),
                    l[2]
                ),
                &t(&assigned_to, l[3]),
                a = l[0],
                b = l[2],
                c = l[3],
            );
        });
    }
    fn print_status_for_category(&self, category_id: u64) {
        let count = self
            .tasks
            .iter()
            .filter(|t| t.category == category_id)
            .count();
        println!(
            "{}: {}",
            self.get_category(category_id).unwrap().name,
            count,
        );
    }
    pub(crate) fn print_status(&self) {
        let tasks_msg = format!("Tasks:\t{}", self.get_unarchived_tasks().len(), );
        println!("{}", tasks_msg.green());
        let users_msg = format!("Users:\t{}", self.users.len());
        println!("{}", users_msg.blue());
        let categories_msg = format!("Categories: {}", self.categories.len());
        println!("{}", categories_msg.yellow());

        println!();
        println!("Tasks per category:");

        for category in &self.categories {
            self.print_status_for_category(category.id);
        }
    }
    pub(crate) fn print_single_task(&self, id: u64) {
        let task = self.tasks.iter().find(|t| t.id == id);

        let width = 60;
        let Some(t) = task else {
            println!("Task with id {} not found", id);
            return;
        };

        utils::print_divider(width);
        utils::print_line_centered(&t.name, width);
        utils::print_divider(width);

        if !t.description.is_empty() {
            for segment in utils::format_description(&t.description, width - 2).iter() {
                utils::print_line_left(segment, width);
            }

            utils::print_divider(width);
        }

        if !t.check_list.is_empty() {
            utils::print_line_left("Checklist:", width);
            for item in &t.check_list {
                let line = match item.checked {
                    true => format!("- [x] {}", item),
                    false => format!("- [ ] {}", item),
                };
                utils::print_line_left(&line, width);
            }
            utils::print_divider(width);
        }
        utils::print_line_left(
            &format!(
                "Category: {}",
                self.categories
                    .iter()
                    .find(|c| c.id == t.category)
                    .unwrap()
                    .name
            ),
            width,
        );

        if t.due_date_utc.is_some() {
            let seconds_till = t.due_date_utc.unwrap() - Utc::now().timestamp();
            let (formatted_due_date, style) = &utils::display_due_date_time(seconds_till);
            utils::print_line_left(
                &format!("Due: {}", formatted_due_date.style(*style)),
                width,
            );
        }
        if self.users.is_empty() {
            utils::print_line_left("No users assigned to task", width);
        } else {
            utils::print_line_left("Users assigned to task:", width);
            self.get_assigned_users(id).iter().for_each(|u| {
                utils::print_line_left(
                    &format!(
                        "- {} <{}>",
                        u.name,
                        u.git_email.to_owned().unwrap_or("no email".to_string())
                    ),
                    width,
                )
            });
        }

        utils::print_divider(width)
    }


    pub(crate) fn print_category(&self, category_id: u64) {
        let mut user_names = HashMap::new();
        for user in &self.users {
            user_names.insert(&user.id, &user.name);
        }

        let category = self
            .categories
            .iter()
            .find(|c| c.id == category_id)
            .unwrap();

        let tasks = self
            .tasks
            .iter()
            .filter(|t| t.category == category.id)
            .collect::<Vec<_>>();
        println!("{}", category.name);
        println!("{}", "-".repeat(category.name.len() * 2));
        if !tasks.is_empty() {
            let header = format!(
                "{:<20} {:<40} {:<20}", // | {:<25} | {:<25}",
                ct("Name", 20).bold(),
                ct("Description", 40).bold(),
                ct("Assigned to", 20).bold(),
            );

            println!("{}", header);

            println!("{}", "-".repeat(4 + 20 + 50));

            for task in tasks {
                let mut assigned_to_string = task
                    .assigned_to
                    .iter()
                    .map(|i| t(&self.get_user(*i).unwrap().name, 10))
                    .collect::<Vec<String>>()
                    .join(", ");

                if assigned_to_string.is_empty() {
                    assigned_to_string = "None".to_string();
                }
                println!(
                    "{:<20}|{:<30}|{:<20}",
                    t(&task.name, 20),
                    t(&task.description, 30),
                    ct(&assigned_to_string, 20),
                );
            }
            println!();
        } else {
            println!("No tasks in this category");
            println!();
        }
    }

    fn unix_time_to_string(time: i64) -> String {
        DateTime::from_timestamp(time, 0)
            .unwrap()
            .to_string()
    }

    pub(crate) fn print_tasks_json(&self) {
        let json_tasks = self
            .tasks
            .iter()
            .map(|t| TaskJson {
                id: t.id,
                name: t.name.to_owned(),
                description: t.description.to_owned(),
                category_id: t.category,
                category: self.get_category(t.category).unwrap().name.to_owned(),
                assigned_to: t
                    .assigned_to
                    .iter()
                    .map(|i| self.get_user(*i).unwrap())
                    .collect::<Vec<User>>(),
                assigned_to_ids: t.assigned_to.to_owned(),
                created_at_utc: Self::unix_time_to_string(t.created_at_utc),
                created_at_utc_unix: t.created_at_utc,
                updated_at_utc: Self::unix_time_to_string(t.updated_at_utc),
                updated_at_utc_unix: t.updated_at_utc,
                archived_at_utc_unix: t.archived_at_utc.unwrap_or(0),
                archived_at_utc: match t.archived_at_utc {
                    Some(archived_at_utc) => Self::unix_time_to_string(archived_at_utc),
                    None => "".to_string(),
                },
                due_date_utc: match t.due_date_utc {
                    Some(due_date_utc) => Self::unix_time_to_string(due_date_utc),
                    None => "".to_string(),
                },
                due_date_utc_unix: t.due_date_utc.unwrap_or(0),
                check_list: t.check_list.to_owned(),
            })
            .collect::<Vec<_>>();

        let json_format = serde_json::to_string_pretty(&json_tasks).unwrap();
        println!("{}", json_format);
    }

    pub(crate) fn print_categories(&self) {
        self.categories.iter().for_each(|c| println!("{}", c.name));
    }

    pub(crate) fn print_users(&self) {
        let users = self.get_users();
        for ele in users {
            match &ele.git_email {
                Some(email) => println!("{} <{}>", ele.name, email),
                None => println!("{} <No email>", ele.name),
            }
        }
    }
    pub(crate) fn print_user_status(&self, user_id: u64) {
        let mut user_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|x| x.assigned_to.iter().any(|u| u == &user_id))
            .collect();

        user_tasks.sort_by(|a, b| b.category.cmp(&a.category));

        for task in &user_tasks {
            println!("{}", task.name.green());
            if !task.description.is_empty() {
                Self::print_description(&task.description);
            }
            println!(
                "Category: {}",
                self.get_category(task.category).unwrap().name
            );
            println!();
        }
    }

    fn print_description(description: &str) {
        println!("{}", "-".repeat(20));
        println!(
            "{}",
            utils::format_description(description, 80).join("\n")
        );
        println!("{}", "-".repeat(20));
    }
}
