mod cli;
mod db;
mod task;

use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;

use cli::Commands;

fn main() -> Result<()> {
    let conn = Connection::open("./tasks.db")?;
    db::init_db(&conn)?;

    let cli = cli::Cli::parse();

    match cli.commands {
        Commands::Add { description } => handle_add(&conn, description)?,
        Commands::List => handle_list(&conn)?,
        Commands::Done { id } => handle_done(&conn, id)?,
        Commands::Undone { id } => handle_undone(&conn, id)?,
        Commands::Remove { id } => handle_remove(&conn, id)?,
    }

    Ok(())
}

fn handle_add(conn: &Connection, description: String) -> Result<()> {
    db::add_task(conn, &description)?;
    println!("A tarefa '{}' foi criada com sucesso.", description);
    Ok(())
}

fn handle_list(conn: &Connection) -> Result<()> {
    let tasks = db::list_tasks(conn)?;
    for task in tasks {
        println!("{} {} - {}", task.status, task.id, task.description);
    }
    Ok(())
}

fn handle_done(conn: &Connection, id: i64) -> Result<()> {
    db::done_task(conn, id)?;
    println!("Tarefa {} concluída.", id);
    Ok(())
}

fn handle_undone(conn: &Connection, id: i64) -> Result<()> {
    db::undone_task(conn, id)?;
    println!("Tarefa {} marcada como pendente.", id);
    Ok(())
}

fn handle_remove(conn: &Connection, id: i64) -> Result<()> {
    db::remove_task(conn, id)?;
    println!("Tarefa {} removida.", id);
    Ok(())
}
