use anyhow::Result;
use clap::{Parser, Subcommand};

mod gen_skel;
mod package;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    task: Task,
}

#[derive(Subcommand)]
enum Task {
    Package,
    GenSkel,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.task {
        Task::Package => package::run(),
        Task::GenSkel => gen_skel::run(),
    }
}
