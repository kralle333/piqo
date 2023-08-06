use crate::utils;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

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

        let header = format!(
            "{:<4} | {:<20} | {:<50} | {:<20} | {:<20} | {:<25} | {:<25}",
            utils::center_align("ID", 4).bold(),
            utils::center_align("Name", 20).bold(),
            utils::center_align("Description", 50).bold(),
            utils::center_align("Category", 20).bold(),
            utils::center_align("Assigned To", 20).bold(),
            utils::center_align("Created At", 25).bold(),
            utils::center_align("Updated At", 25).bold(),
        );
        println!("{}", header);
        println!("{}", "-".repeat(180));
        for t in &tasks {
            let assigned_to_string = t
                .assigned_to
                .iter()
                .map(|i| self.get_user(*i).unwrap().name())
                .collect::<Vec<&str>>()
                .join(", ");
            let row = format!(
                "{:<4} | {:<20} | {:<50} | {:<20} | {:<20} | {:<25} | {:<25}",
                t.id,
                utils::truncate(&t.name, 20),
                utils::truncate(&t.description, 50),
                utils::center_align(category_names.get(&t.category).unwrap(), 20),
                utils::center_align(&utils::truncate(&assigned_to_string, 20), 20),
                chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(t.created_at_utc, 0).unwrap(),
                    chrono::Utc
                )
                .to_string(),
                chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(t.updated_at_utc, 0).unwrap(),
                    chrono::Utc
                )
                .to_string(),
            );
            println!("{}", row);
        }
    }

    pub(crate) fn print_single_task(&self, id: u64) {
        let task = self.tasks.iter().find(|t| t.id == id);

        let width = 60;

        match task {
            Some(t) => {
                utils::print_divider(width);
                utils::print_line_centered(&t.name, width);
                utils::print_divider(width);
                utils::print_line_left(&t.description, width);

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
                    60,
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
                    self.users.iter().for_each(|u| {
                        utils::print_line_left(&format!("- {} | {}", u.id, u.name), 60)
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
                    "{:<4} | {:<20} | {:<50}",
                    utils::center_align("ID", 4).bold(),
                    utils::center_align("Name", 20).bold(),
                    utils::center_align("Description", 50).bold()
                );

                println!("{}", header);

                println!("{}", "-".repeat(4 + 20 + 50));

                for task in tasks {
                    println!(
                        "{:<4} | {:<20} | {:<50}",
                        task.id,
                        utils::truncate(&task.name, 20),
                        task.description
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
}
