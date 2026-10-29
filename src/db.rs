use anyhow::Result;
use rusqlite::{Connection, params};

use crate::task::{Task, TaskStatus};

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("tarefa com id {0} não encontrada")]
    NotFound(i64),
    #[error("erro no banco de dados {0}")]
    Sqlite(#[from] rusqlite::Error),
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            status      TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn add_task(conn: &Connection, description: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO tasks (description, status) VALUES (?1, ?2)",
        params![description, TaskStatus::Pending.as_str()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, status FROM tasks ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        let status_str: String = row.get(2)?;
        let status: TaskStatus = TaskStatus::from_str(&status_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, e.into())
        })?;

        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            status,
        })
    })?;

    let mut tasks = Vec::new();
    for task in rows {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn done_task(conn: &Connection, id: i64) -> Result<(), DbError> {
    let rows_affected = conn.execute(
        "UPDATE tasks SET status = ?1 WHERE id = ?2",
        params![TaskStatus::Done.as_str(), id],
    )?;

    ensure_affected(id, rows_affected)
}

pub fn undone_task(conn: &Connection, id: i64) -> Result<(), DbError> {
    let rows_affected = conn.execute(
        "UPDATE tasks SET status = ?1 WHERE id = ?2",
        params![TaskStatus::Pending.as_str(), id],
    )?;

    ensure_affected(id, rows_affected)
}

pub fn remove_task(conn: &Connection, id: i64) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;

    ensure_affected(id, rows_affected)
}

fn ensure_affected(id: i64, rows_affected: usize) -> Result<(), DbError> {
    if rows_affected == 0 {
        return Err(DbError::NotFound(id));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_list_tasks() {
        // 1. Arrange (preparar): banco limpo em memória, com a tabela criada
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        // 2. Act (ação): a operação
        add_task(&conn, "tarefa teste").unwrap();

        // 3. Assert (verificar): o resultado é o esperado?
        let tasks = list_tasks(&conn).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "tarefa teste");
        assert_eq!(tasks[0].status, TaskStatus::Pending)
    }

    #[test]
    fn done_and_undone_tasks() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let id = add_task(&conn, "tarefa teste").unwrap();

        done_task(&conn, id).unwrap();
        let tasks = list_tasks(&conn).unwrap();
        assert_eq!(tasks[0].status, TaskStatus::Done);

        undone_task(&conn, id).unwrap();
        let tasks = list_tasks(&conn).unwrap();
        assert_eq!(tasks[0].status, TaskStatus::Pending)
    }

    #[test]
    fn done_task_with_invalid_id_returns_not_found() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = done_task(&conn, 999);

        assert!(result.is_err());
        assert!(matches!(result, Err(DbError::NotFound(999))));
    }
}
