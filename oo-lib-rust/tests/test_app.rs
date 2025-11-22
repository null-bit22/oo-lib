use oo_lib_rust::v1::member::Member;
use oo_lib_rust::v1::catalog::Catalog;
use oo_lib_rust::v1::items::{Book, Dvd};

fn setup_sample() -> Catalog {
    let mut cat = Catalog::new();
    cat.add(Box::new(Book::new("B1", "Rust for Humans"))).unwrap();
    cat.add(Box::new(Book::new("B2", "Pythonic Patterns"))).unwrap();
    cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
    cat
}

#[test]
fn test_add_and_get() {
    let cat = setup_sample();
    assert_eq!(cat.get("B1").unwrap().get_title(), "Rust for Humans");
    assert_eq!(cat.get("D1").unwrap().days_allowed(), 7);
}

#[test]
fn test_duplicate_id_rejected() {
    let mut cat = setup_sample();
    let result = cat.add(Box::new(Book::new("B1", "Duplicate")));
    assert!(result.is_err());
}

#[test]
fn test_member_borrow_and_return() {
    let mut m = Member::new("Clark");
    m.borrow("B1").unwrap();
    let mut vec = vec!["B1".to_string()];
    assert_eq!(m.borrowed_ids(), &vec);
    
    m.return_item("B1").unwrap();
    vec.pop(); // []
    assert_eq!(m.borrowed_ids(), &vec); // [] == [] 
}

#[test]
fn test_member_cannot_borrow_twice() {
    let mut m = Member::new("Adam");
    m.borrow("K1").unwrap();
    let result = m.borrow("K1");
    assert!(result.is_err());
}

#[test]
fn test_member_limit_enforced() {
    let mut m = Member::new("Adam");
    for i in 0..5 {
        m.borrow(&format!("K{}", i)).unwrap();
    }
    let result = m.borrow("K5");
    assert!(result.is_err());
}

#[test]
fn test_details_lines() {
    let cat = setup_sample();
    let mut m = Member::new("Adam");
    m.borrow("B2").unwrap();
    let lines = cat.details_for(&m.borrowed_ids());
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].0, "B2".to_string());
    assert_eq!(lines[0].1, "Pythonic Patterns".to_string());
    assert_eq!(lines[0].2, 14);
}

#[test]
fn test_member_cannot_return_unborrowed_id() {
    let mut m = Member::new("Adam");
    m.borrow("M1").unwrap();
    let result = m.return_item("Z10");
    assert!(result.is_err());
}

#[test]
fn test_member_cannot_borrow_items_not_in_catalog() {
    let cat = setup_sample();
    let mut m = Member::new("Zeke");
    m.borrow("A24").unwrap();
    assert_eq!(m.is_borrowed_list_valid(&cat), false);
}
