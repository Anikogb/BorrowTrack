use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loan {
    pub id: u32,
    pub owner_id: u32,
    pub borrower_id: u32,
    pub item_id: u32,
    pub borrow_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub returned: bool,
}
