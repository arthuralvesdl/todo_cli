use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-cli", about = "Gerenciador de tarefas simples")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adiciona uma nova tarefa
    Add { description: String },
    /// Lista todas as tarefas
    List,
    /// Marca uma tarefa como concluída
    Done { id: i64 },
    /// Marca uma tarefa como não concluída
    Undone { id: i64 },
    /// Remove uma tarefa
    Remove { id: i64 },
}
