use std::fs;
use std::path::Path;
use serde_json;
use crate::models::{User, Item, Loan};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Database {
    pub users: Vec<User>,
    pub items: Vec<Item>,
    pub loans: Vec<Loan>,
}

pub fn load_db<P: AsRef<Path>>(path: P) -> Result<Database, Box<dyn std::error::Error>> {
    if !path.as_ref().exists() {
        return Ok(Database::default());
    }
    let data = fs::read_to_string(path)?;
    let db = serde_json::from_str(&data)?;
    Ok(db)
}

pub fn save_db<P: AsRef<Path>>(path: P, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(db)?;
    fs::write(path, data)?;
    Ok(())
}
