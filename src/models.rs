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
    pub due_date_utc: Option<i64>,
    pub assigned_to: Vec<u64>,
    pub check_list: Vec<CheckListItem>,
    pub last_check_list_index: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CheckListItem {
    pub index: u64,
    pub name: String,
    pub checked: bool,
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
    pub due_date_utc_unix: i64,
    pub due_date_utc: String,
    pub assigned_to_ids: Vec<u64>,
    pub assigned_to: Vec<User>,
    pub check_list: Vec<CheckListItem>,
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

impl Display for CheckListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Project {
    pub(crate) fn new(name: String) -> Self {
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

    pub(crate) fn add_task(&mut self, name: String) -> u64 {
        let id = utils::get_unused_id(self.tasks.iter().map(|i| i.id).collect());
        let created_at_utc = chrono::Utc::now().timestamp();
        let updated_at_utc = chrono::Utc::now().timestamp();
        let category = self.default_category;
        let task = Task {
            id,
            name,
            description: String::new(),
            category,
            created_at_utc,
            updated_at_utc,
            archived_at_utc: None,
            assigned_to: vec![],
            due_date_utc: None,
            check_list: vec![],
            last_check_list_index: 0,
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
        self.tasks
            .retain_mut(|t| t.assigned_to.iter().all(|u| u != &ele.id));
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

    pub(crate) fn edit_user(&mut self, id: u64, name: &str, email: Option<String>) {
        self.users.iter_mut().find(|u| u.id == id).unwrap().name = name.to_string();
        self.users
            .iter_mut()
            .find(|u| u.id == id)
            .unwrap()
            .git_email = email;
    }

    pub(crate) fn add_checklist_item(&mut self, task_id: u64, checklist_item_name: String) {
        let task = self.tasks.iter_mut().find(|t| t.id == task_id).unwrap();

        let next_index = task.last_check_list_index + 1;

        task.check_list.push(CheckListItem {
            index: next_index,
            name: checklist_item_name,
            checked: false,
        });
        task.last_check_list_index = next_index;
    }

    pub(crate) fn remove_checklist_item(&mut self, task_id: u64, check_list_index: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .unwrap()
            .check_list
            .retain(|c| c.index != check_list_index);
    }
    pub(crate) fn get_task_checklist(&self, task_id: u64) -> Vec<CheckListItem> {
        self.tasks
            .iter()
            .find(|t| t.id == task_id)
            .unwrap()
            .check_list
            .iter()
            .map(|c| c.to_owned())
            .collect()
    }

    pub(crate) fn set_task_due_date(&mut self, id: u64, due_date: i64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .unwrap()
            .due_date_utc = Some(due_date);
    }

    pub(crate) fn clear_task_due_date(&mut self, id: u64) {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .unwrap()
            .due_date_utc = None;
    }

    pub(crate) fn get_task_due_time(&self, id: u64) -> Option<i64> {
        self.tasks.iter().find(|t| t.id == id).unwrap().due_date_utc
    }
}
