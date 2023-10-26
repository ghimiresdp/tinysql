use super::{db::Database, io::input};

pub(crate) struct Sql {
    db: Option<Database>,
}

impl Sql {
    fn query(&self, query: String) {
        println!();
        match &self.db {
            Some(db) => db.query(query),
            None => {
                // queries without db connection such as create database
                todo!();
            }
        }
    }
    fn command(&self, cmd: String) {
        println!("command: {cmd}");
    }
    pub(crate) fn new() -> Self {
        Self { db: None }
    }
    pub(crate) fn connect(&mut self, db_name: String) {
        let db = Database::select(db_name);
        self.db = Some(db);
    }
    pub(crate) fn display_shell(&self) {
        loop {
            let query = input(
                format!(
                    "\n[ {} ] > ",
                    match &self.db {
                        Some(db) => db.name.clone(),
                        None => "".to_owned(),
                    }
                )
                .as_str(),
            )
            .trim()
            .to_string();
            if query.to_lowercase() == "exit" {
                break;
            }
            if query.starts_with(".") {
                self.command(query);
            } else {
                self.query(query);
            }
        }
    }
}

enum DotCommands {
    EXIT,
    LIST,
    USE(String)
}
