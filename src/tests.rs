#[cfg(test)]
mod tests {
    use crate::BorrowTrack;
    use crate::models::{User, Item, Loan};
    use std::path::PathBuf;
    use chrono::{Utc, Duration};
    use tempfile::tempdir;

    fn setup() -> (BorrowTrack, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_data.json");
        let bt = BorrowTrack::new(db_path).unwrap();
        (bt, dir)
    }

    #[test]
    fn test_add_user_and_item() {
        let (mut bt, _dir) = setup();
        
        bt.add_user("Alice".to_string());
        bt.add_item("Book".to_string(), "A nice book".to_string());
        
        assert_eq!(bt.search_by_user("Alice").len(), 1);
        assert_eq!(bt.search_by_item("Book").len(), 1);
    }

    #[test]
    fn test_loan_management() {
        let (mut bt, _dir) = setup();
        
        bt.add_user("Alice".to_string()); // ID 1
        bt.add_user("Bob".to_string());   // ID 2
        bt.add_item("Book".to_string(), "A nice book".to_string()); // ID 1
        
        let now = Utc::now();
        let due = now + Duration::days(7);
        
        bt.create_loan(1, 2, 1, now, due);
        
        let active = bt.list_active_loans();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].item_id, 1);
        assert_eq!(active[0].returned, false);
        
        let loan_id = active[0].id;
        assert!(bt.mark_returned(loan_id));
        
        assert_eq!(bt.list_active_loans().len(), 0);
    }

    #[test]
    fn test_overdue_loans() {
        let (mut bt, _dir) = setup();
        
        bt.add_user("Alice".to_string());
        bt.add_user("Bob".to_string());
        bt.add_item("Book".to_string(), "Description".to_string());
        
        let now = Utc::now();
        // Create an overdue loan (due 1 day ago)
        let past_due = now - Duration::days(1);
        bt.create_loan(1, 2, 1, now - Duration::days(2), past_due);
        
        // Create a non-overdue loan (due in 1 day)
        let future_due = now + Duration::days(1);
        bt.create_loan(1, 2, 1, now, future_due);
        
        let overdue = bt.overdue_loans();
        assert_eq!(overdue.len(), 1);
        assert!(overdue[0].due_date < now);
    }
}
