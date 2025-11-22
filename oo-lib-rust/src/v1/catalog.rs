use crate::v1::items::{Item};
use std::collections::HashMap;

pub struct Catalog {
    catalog: HashMap<String, Box<dyn Item>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            catalog: HashMap::new(), // Start with empty HashMap
        }
    }

    pub fn add(&mut self, item: Box<dyn Item>) -> Result<(), String> {
        let key: &str = item.get_id();

        // Check for duplicate Ids before adding to catalog
        if self.catalog.contains_key(key) {
            Err(format!("Duplicate entry for id: {}", key))
        } else { 
            self.catalog.insert(key.to_string(), item);
            Ok(()) 
        }
    }

    pub fn details_for(&self, borrowed_ids: &Vec<String>) -> Vec<(String, String, u8)> {
        let mut entries: Vec<(String, String, u8)> = Vec::new();

        for id in borrowed_ids {
            let item = self.catalog.get(id).unwrap();
            let tup = (item.get_id().to_string(), 
                item.get_title().to_string(), 
                item.days_allowed());
            entries.push(tup);
        }
        entries
    }

    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Result<&Box<dyn Item>, String> {
        if self.catalog.contains_key(id) {
            Ok(self.catalog.get(id).unwrap())
        } else {
            Err(format!("No entry found for id: {}", id))
        }
    }
}
