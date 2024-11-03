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

impl Display for CheckListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
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
