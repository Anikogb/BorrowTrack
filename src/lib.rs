pub mod models;
pub mod db;

use db::{Database, load_db, save_db};
use std::path::PathBuf;

pub struct BorrowTrack {
    db: Database,
    db_path: PathBuf,
}

impl BorrowTrack {
    pub fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let db = load_db(&db_path)?;
        Ok(Self { db, db_path })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        save_db(&self.db_path, &self.db)
    }

    pub fn add_user(&mut self, name: String) {
        let id = (self.db.users.len() as u32) + 1;
        self.db.users.push(models::User { id, name });
    }

    pub fn add_item(&mut self, name: String, description: String) {
        let id = (self.db.items.len() as u32) + 1;
        self.db.items.push(models::Item { id, name, description });
    }

    pub fn create_loan(
        &mut self,
        owner_id: u32,
        borrower_id: u32,
        item_id: u32,
        borrow_date: chrono::DateTime<chrono::Utc>,
        due_date: chrono::DateTime<chrono::Utc>,
    ) {
        let id = (self.db.loans.len() as u32) + 1;
        self.db.loans.push(models::Loan {
            id,
            owner_id,
            borrower_id,
            item_id,
            borrow_date,
            due_date,
            returned: false,
        });
    }

    pub fn mark_returned(&mut self, loan_id: u32) -> bool {
        if let Some(loan) = self.db.loans.iter_mut().find(|l| l.id == loan_id) {
            loan.returned = true;
            true
        } else {
            false
        }
    }

    pub fn list_active_loans(&self) -> Vec<&models::Loan> {
        self.db.loans.iter().filter(|l| !l.returned).collect()
    }

    pub fn overdue_loans(&self) -> Vec<&models::Loan> {
        let now = chrono::Utc::now();
        self.db.loans.iter()
            .filter(|l| !l.returned && l.due_date < now)
            .collect()
    }

    pub fn search_by_user(&self, user_name: &str) -> Vec<&models::User> {
        self.db.users.iter()
            .filter(|u| u.name.to_lowercase().contains(&user_name.to_lowercase()))
            .collect()
    }

    pub fn search_by_item(&self, item_name: &str) -> Vec<&models::Item> {
        self.db.items.iter()
            .filter(|i| i.name.to_lowercase().contains(&item_name.to_lowercase()))
            .collect()
    }
}
