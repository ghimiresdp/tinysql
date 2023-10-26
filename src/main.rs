use clap::{Parser, Subcommand};
mod core;

use core::db::Database;
use crate::core::Sql;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
    New { name: Option<String> },
    Drop { name: Option<String> },
    Use { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let mut sql = Sql::new();

    match &cli.command {
        Some(Commands::List) => {
            println!("DATABASES\n\n");
            Database::list();
        }
        Some(Commands::New { name }) => {
            match name {
                Some(name) => Database::create(name.to_owned()),
                None => panic!("The name of the database is required"),
            };
        }
        Some(Commands::Drop { name }) => {
            println!("Deleting Database '{name:?}'.")
        }
        Some(Commands::Use { name }) => match name {
            Some(name) => {
                sql.connect(name.to_owned());
                sql.display_shell();
            }
            None => panic!("No database is selected"),
        },
        None => {
            // when no arguments are passed, sql shell is displayed
            sql.display_shell();
        }
    }
}
