use std::{fs, io};

use super::Sql;

mod dtype;
mod table;

pub(crate) struct Database {
    pub(crate) name: String,
}

impl Database {
    pub(crate) fn create(name: String) -> Self {
        let path = format!("db/{name}");
        match fs::create_dir_all(path.clone()) {
            Ok(_) => {
                println!("Creating Database {name}");
                match fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(format!("{path}/index.json"))
                {
                    Ok(_) => {
                        print!("Created a database with name \"{name}\"");
                        Self { name: name }
                    }
                    Err(err) => {
                        if err.kind() == io::ErrorKind::AlreadyExists {
                            panic!("The Database with name '{name}' already exists")
                        } else {
                            panic!("Could not create a database named '{name}'");
                        }
                    }
                }
            }
            Err(_) => panic!("Could not create a database named \"{name}\""),
        }
    }
    pub(crate) fn select(name: String) -> Self {
        Self { name }
    }
    pub(crate) fn query(&self, query: String){
        Sql::query(query)
    }
}
