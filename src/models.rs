use crate::utils;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::utils::center_align as c;
use crate::utils::left_align as l;
use crate::utils::truncate as t;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Task {
    id: u64,
    name: String,
    description: String,
    category: u64,
    created_at_utc: i64,
    updated_at_utc: i64,
    archieved_at_utc: Option<i64>,
    assigned_to: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Project {
    name: String,
    default_category: u64,
    categories: Vec<Category>,
    tasks: Vec<Task>,
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Category {
    id: u64,
    name: String,
}

impl Category {
    pub(crate) fn id(&self) -> &u64 {
        &self.id
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    pub id: u64,
    pub git_email: Option<String>,
    pub name: String,
}
impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn git_email(&self) -> Option<String> {
        self.git_email.to_owned()
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.id, self.name)
    }
}

impl Task {
    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }
    pub(crate) fn id(&self) -> u64 {
        self.id
    }
}

impl Project {
    pub fn new(name: String) -> Self {
        Project {
            name,
            default_category: 0,
            categories: vec![],
            tasks: vec![],
            users: vec![],
        }
    }

    pub(crate) fn categories(&self) -> &Vec<Category> {
        &self.categories
    }

    pub(crate) fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn create_category(&self, name: &str) -> Category {
        let id = utils::get_unused_id(self.categories.iter().map(|i| i.id).collect());
        Category {
            id,
            name: name.to_string(),
        }
    }

    pub(crate) fn add_default_category(&mut self, name: &str) {
        let s = self.create_category(name);
        self.default_category = s.id;
        self.categories.push(s);
    }

    pub(crate) fn add_category(&mut self, name: &str) {
        self.categories.push(self.create_category(name))
    }

    pub(crate) fn add_task(&mut self, name: String, description: String) {
        let id = utils::get_unused_id(self.tasks.iter().map(|i| i.id).collect());
        let created_at_utc = chrono::Utc::now().timestamp();
        let updated_at_utc = chrono::Utc::now().timestamp();
        let category = self.default_category;
        let task = Task {
            id,
            name,
            description,
            category,
            created_at_utc,
            updated_at_utc,
            archieved_at_utc: None,
            assigned_to: vec![],
        };
        self.tasks.push(task);
    }

    pub(crate) fn print_tasks(&self) {
        println!("Tasks for Project: {}", self.name);
        println!();
        // print tasks with a columns: name, description, category, assigned_to
        // also sort the tasks by category

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

        let lengths = vec![15, 30, 12, 20];

        let p = |a, b, c, d| println!("{:<15}|{:<30}|{:<12}|{:<20}", a, b, d, c);
        p(
            c("Name", lengths[0]).bold().to_string(),
            c("Description", lengths[1]).bold().to_string(),
            c("Category", lengths[2]).bold().to_string(),
            c("Assigned To", lengths[3]).bold().to_string(),
        );

        println!("{}", "-".repeat(80));

        for task in &tasks {
            let mut assigned_to_string = task
                .assigned_to
                .iter()
                .map(|i| t(self.get_user(*i).unwrap().name(), 10))
                .collect::<Vec<String>>()
                .join(", ");

            if assigned_to_string.is_empty() {
                assigned_to_string = "None".to_string();
            }

            p(
                t(&task.name, lengths[0]),
                t(&task.description, lengths[1]),
                c(category_names.get(&task.category).unwrap(), lengths[2]),
                c(&t(&assigned_to_string, lengths[3]), lengths[3]),
            );
        }
    }

    pub(crate) fn print_single_task(&self, id: u64) {
        let task = self.tasks.iter().find(|t| t.id == id);

        let width = 80;

        match task {
            Some(t) => {
                utils::print_divider(width);
                utils::print_line_centered(&t.name, width);
                utils::print_divider(width);

                for segment in utils::to_segments(&t.description, width - 2).iter() {
                    utils::print_line_left(segment, width);
                }

                utils::print_divider(width);
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
                                u.name(),
                                u.git_email().unwrap_or("no email".to_string())
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

    pub(crate) fn print_status(&self) {
        println!("Status for Project: {}", self.name);
        println!();

        let mut tasks_dict = HashMap::new();
        for category in &self.categories {
            tasks_dict.insert(category.id, Vec::new());
        }

        for task in &self.tasks {
            tasks_dict.get_mut(&task.category).unwrap().push(task);
        }

        let mut user_names = HashMap::new();
        for user in &self.users {
            user_names.insert(&user.id, &user.name);
        }
        // for category in &self.categories {
        //     let tasks = tasks_dict.get(&category.id).unwrap();
        //     print!("{}: {} ", category.name, tasks.len())
        // }
        // println!();
        // Center-align helper function

        for category in &self.categories {
            let tasks = tasks_dict.get(&category.id).unwrap();
            println!("\x1b[32m{}:\x1b[0m", category.name); // Green background for category name
            println!("{}", "=".repeat(category.name.len() * 2));
            if !tasks.is_empty() {
                println!();

                let header = format!(
                    "{:<20}|{:<40}|{:<20}", // | {:<25} | {:<25}",
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
                        .map(|i| t(self.get_user(*i).unwrap().name(), 10))
                        .collect::<Vec<String>>()
                        .join(", ");

                    if assigned_to_string.is_empty() {
                        assigned_to_string = "None".to_string();
                    }
                    println!(
                        "{:<20}|{:<30}|{:<20}",
                        t(&task.name, 20),
                        t(&task.description, 30),
                        l(&assigned_to_string, 20),
                    );
                }
                println!();
            } else {
                println!("No tasks in this category");
                println!();
            }
        }
    }

    pub(crate) fn archieve_task(&mut self, id: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .unwrap()
            .archieved_at_utc = Some(chrono::Utc::now().timestamp());
    }

    pub(crate) fn move_task(&mut self, task_id: u64, category_id: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .unwrap()
            .category = category_id;
    }

    pub(crate) fn remove_category(&mut self, id: u64) {
        self.categories.retain_mut(|c| c.id != id)
    }

    pub(crate) fn get_category(&self, id: u64) -> Option<&Category> {
        self.categories.iter().find(|c| c.id == id)
    }

    pub(crate) fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub(crate) fn remove_user(&mut self, ele: &User) {
        self.users.retain_mut(|u| u.id != ele.id);
    }

    pub(crate) fn assign_task(&mut self, user_id: u64, task_id: u64) {
        let already_assigned = self
            .tasks
            .iter()
            .find(|t| t.id == task_id)
            .unwrap()
            .assigned_to
            .iter()
            .any(|u| u == &user_id);
        if already_assigned {
            return;
        }

        self.tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .unwrap()
            .assigned_to
            .push(user_id);
    }

    pub(crate) fn unassign_task(&mut self, user_id: u64, task_id: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .unwrap()
            .assigned_to
            .retain(|u| u != &user_id);
    }

    pub(crate) fn add_user(&mut self, name: &str, git_email: &str) {
        let id = utils::get_unused_id(self.users.iter().map(|u| u.id).collect());

        self.users.push(User {
            id,
            name: name.to_string(),
            git_email: Some(git_email.to_string()),
        });
    }

    pub(crate) fn get_user(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub(crate) fn get_assigned_users(&self, id: u64) -> Vec<&User> {
        let task = self.tasks.iter().find(|t| t.id == id).unwrap();
        let mut users = Vec::new();
        for user_id in &task.assigned_to {
            users.push(self.get_user(*user_id).unwrap().to_owned());
        }
        users
    }

    pub(crate) fn print_categories(&self) {
        self.categories.iter().for_each(|c| {
            println!("{} | {}", c.id, c.name);
        })
    }

    pub(crate) fn edit_category(&mut self, category_id: u64, new_name: &str) {
        self.categories
            .iter_mut()
            .find(|c| c.id == category_id)
            .unwrap()
            .name = new_name.to_string();
    }

    pub(crate) fn edit_task_name(&mut self, id: u64, new_name: String) {
        self.tasks.iter_mut().find(|t| t.id == id).unwrap().name = new_name;
    }

    pub(crate) fn edit_task_description(&mut self, id: u64, new_description: String) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .unwrap()
            .description = new_description;
    }
}
