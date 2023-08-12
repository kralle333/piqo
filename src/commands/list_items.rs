use std::fmt::{self, Display, Formatter};

pub struct TaskItem {
    pub id: u64,
    pub name: String,
    pub category: Option<String>,
}

impl Display for TaskItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.category.is_none() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} ({})", self.name, self.category.as_ref().unwrap())
        }
    }
}

pub struct CategoryItem {
    pub id: u64,
    pub name: String,
    pub not_deletable: bool,
}

impl Display for CategoryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.not_deletable {
            write!(f, "{} (not deletable - has tasks)", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub struct UserItem {
    pub name: String,
    pub git_email: String,
}

impl Display for UserItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.git_email)
    }
}
