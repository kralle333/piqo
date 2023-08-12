use crate::utils;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Task {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub category: u64,
    pub created_at_utc: i64,
    pub updated_at_utc: i64,
    pub archived_at_utc: Option<i64>,
    pub assigned_to: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TaskJson {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub category_id: u64,
    pub category: String,
    pub created_at_utc_unix: i64,
    pub created_at_utc: String,
    pub updated_at_utc_unix: i64,
    pub updated_at_utc: String,
    pub archived_at_utc_unix: i64,
    pub archived_at_utc: String,
    pub assigned_to_ids: Vec<u64>,
    pub assigned_to: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Project {
    pub name: String,
    pub default_category: u64,
    pub categories: Vec<Category>,
    pub tasks: Vec<Task>,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Category {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct User {
    pub id: u64,
    pub git_email: Option<String>,
    pub name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
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

    pub(crate) fn add_task(&mut self, name: String, description: String) -> u64 {
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
            archived_at_utc: None,
            assigned_to: vec![],
        };
        self.tasks.push(task);
        id
    }

    pub(crate) fn archive_task(&mut self, id: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .unwrap()
            .archived_at_utc = Some(chrono::Utc::now().timestamp());
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

    pub(crate) fn get_category_name(&self, id: u64) -> Option<String> {
        self.categories
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.name.clone())
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

    pub(crate) fn get_user(&self, id: u64) -> Option<User> {
        self.users.iter().find(|u| u.id == id).map(|u| u.to_owned())
    }

    pub(crate) fn get_assigned_users(&self, id: u64) -> Vec<User> {
        let task = self.tasks.iter().find(|t| t.id == id).unwrap();
        let mut users = Vec::new();
        for user_id in &task.assigned_to {
            users.push(self.get_user(*user_id).unwrap().to_owned());
        }
        users
    }
    pub(crate) fn get_unarchived_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.archived_at_utc.is_none())
            .collect()
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

    pub(crate) fn remove_task(&mut self, id: u64) {
        self.tasks.retain(|t| t.id != id)
    }

    pub(crate) fn get_user_by_email(&self, email: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|u| u.git_email.as_ref().unwrap() == email)
    }

    pub(crate) fn get_task_description(&self, id: u64) -> &str {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .unwrap()
            .description
            .as_str()
    }
}
