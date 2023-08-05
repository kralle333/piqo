use std::fmt::{self, Display, Formatter};

pub struct TaskItem {
    pub id: u64,
    pub name: String,
}

impl Display for TaskItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.id, self.name)
    }
}

pub struct CategoryItem {
    pub id: u64,
    pub name: String,
}

impl Display for CategoryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.id, self.name)
    }
}

pub struct UserItem {
    pub name: String,
    pub git_email: String,
}

impl Display for UserItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.name, self.git_email)
    }
}
