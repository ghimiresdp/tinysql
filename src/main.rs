use clap::{Parser, Subcommand};
mod core;

use core::{db::Database, io::input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

    match &cli.command {
        Commands::List => {
            println!("DATABASES\n\n")
        }
        Commands::New { name } => {
            match name {
                Some(name) => Database::create(name.to_owned()),
                None => panic!("The name of the database is required"),
            };
        }
        Commands::Drop { name } => {
            println!("Deleting Database '{name:?}'.")
        }
        Commands::Use { name } => {
            match name{
                Some(name) => {
                    let db = Database::select(name.to_owned());
                    loop {
                        let query = input(format!("\n[ {} ]# ", db.name).as_str()).trim().to_string();
                        if query.to_lowercase() == "exit" {
                            break;
                        }
                        db.query(query);
                    }
                },
                None => panic!("No database is selected"),
            }

        }
    }
}
