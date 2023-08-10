use chrono::NaiveDateTime;
use owo_colors::OwoColorize;
use std::collections::HashMap;

use crate::models::{TaskJson, User};
use crate::utils::truncate as t;
use crate::utils::{self, left_align as l};
use crate::{models::Project, utils::center_align as c};

impl Project {
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

        let l = vec![36, 0, 12, 30];

        let header_0 = &c("Name", l[0]).bold().to_string();
        // let header_1 = &c("Description", l[1]);
        let header_2 = &c("Category", l[2]).bold().to_string();
        let header_3 = &c("Assigned To", l[3]).bold().to_string();
        println!("{:<36}|{:<12}|{:<30}", header_0, header_2, header_3,);

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

            println!(
                "{:<36}|{:<12}|{:<30}",
                &t(&task.name, l[0]),
                // &t(&task.description, l[1]),
                &c(
                    &t(
                        &self.get_category(task.category).unwrap().name.to_string(),
                        l[2]
                    ),
                    l[2]
                ),
                &t(&assigned_to, l[3])
            );
        });
    }
    pub(crate) fn print_single_task(&self, id: u64) {
        let task = self.tasks.iter().find(|t| t.id == id);

        let width = 60;

        match task {
            Some(t) => {
                utils::print_divider(width);
                utils::print_line_centered(&t.name, width);
                utils::print_divider(width);

                if !t.description.is_empty() {
                    for segment in utils::to_segments(&t.description, width - 2).iter() {
                        utils::print_line_left(segment, width);
                    }

                    utils::print_divider(width);
                }
                utils::print_line_left(
                    &format!(
                        "Categories: {}",
                        self.categories
                            .iter()
                            .find(|c| c.id == t.category)
                            .unwrap()
                            .name
                    ),
                    width,
                );
                let created_at =
                    chrono::NaiveDateTime::from_timestamp_opt(t.created_at_utc, 0).unwrap();

                utils::print_line_left(
                    &format!(
                        "Created At: {}",
                        chrono::DateTime::<chrono::Utc>::from_utc(
                            created_at.to_owned(),
                            chrono::Utc
                        )
                    ),
                    width,
                );

                let modified_at = &format!(
                    "Updated At: {}",
                    chrono::DateTime::<chrono::Utc>::from_utc(
                        chrono::NaiveDateTime::from_timestamp_opt(t.updated_at_utc, 0).unwrap(),
                        chrono::Utc
                    )
                );
                utils::print_line_left(modified_at.as_str(), width);

                if self.users.is_empty() {
                    utils::print_line_left("No users assigned to task", width);
                } else {
                    utils::print_line_left("Users assigned to task:", width);
                    self.get_assigned_users(id).iter().for_each(|u| {
                        utils::print_line_left(
                            &format!(
                                " • {} <{}>",
                                u.name,
                                u.git_email.to_owned().unwrap_or("no email".to_string())
                            ),
                            width,
                        )
                    });
                }

                utils::print_divider(width)
            }
            None => println!("Task with id {} not found", id),
        }
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
                c("Name", 20).bold(),
                c("Description", 40).bold(),
                c("Assigned to", 20).bold(),
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
                    c(&assigned_to_string, 20),
                );
            }
            println!();
        } else {
            println!("No tasks in this category");
            println!();
        }
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
                    .collect::<Vec<crate::models::User>>(),
                assigned_to_ids: t.assigned_to.to_owned(),
                created_at_utc: NaiveDateTime::from_timestamp_opt(t.created_at_utc, 0)
                    .unwrap()
                    .to_string(),
                created_at_utc_unix: t.created_at_utc,
                updated_at_utc: NaiveDateTime::from_timestamp_opt(t.updated_at_utc, 0)
                    .unwrap()
                    .to_string(),
                updated_at_utc_unix: t.updated_at_utc,
                archieved_at_utc_unix: t.archieved_at_utc.unwrap_or(0),
                archieved_at_utc: match t.archieved_at_utc {
                    Some(archieved_at_utc) => {
                        NaiveDateTime::from_timestamp_opt(archieved_at_utc, 0)
                            .unwrap()
                            .to_string()
                    }

                    None => "".to_string(),
                },
            })
            .collect::<Vec<_>>();

        let json_format = serde_json::to_string_pretty(&json_tasks).unwrap();
        println!("{}", json_format);
    }

    pub(crate) fn print_categories(&self) {
        self.categories.iter().for_each(|c| println!("{}", c.name));
    }

    pub(crate) fn print_status(&self) {
        self.tasks.iter().for_each(|t| {
            self.print_single_task(t.id);
            println!()
        });
    }
}
