use std::fmt;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Done,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Done => "done",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "pending" => Ok(TaskStatus::Pending),
            "done" => Ok(TaskStatus::Done),
            other => anyhow::bail!("status desconhecido no banco: '{}'", other),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkbox = match self {
            TaskStatus::Pending => "[]",
            TaskStatus::Done => "[x]",
        };
        write!(f, "{}", checkbox)
    }
}
