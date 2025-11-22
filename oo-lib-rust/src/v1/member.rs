use crate::v1::catalog::Catalog;

pub struct Member {
    name: String,
    borrowed_ids: Vec<String>,
    max_borrow_limit: usize,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            borrowed_ids: Vec::new(),
            max_borrow_limit: 5,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn borrowed_ids(&self) -> &Vec<String> {
        &self.borrowed_ids
    }

    pub fn borrow(&mut self, id: &str) -> Result<(), String> {
        // Check for max borrow limit
        if self.borrowed_ids.len() == self.max_borrow_limit {
            Err(format!("Max borrow limit reached!"))
        
        // Check for duplicate borrows
        } else if self.borrowed_ids.contains(&id.to_string()) {
            Err(format!("Duplicate borrow for id: {}", id))

        // Else add to list
        } else {
            self.borrowed_ids.push(id.to_string());
            Ok(())
        }
    }

    pub fn return_item(&mut self, id: &str) -> Result<(), String> {
        if self.borrowed_ids.contains(&id.to_string()) {
            self.borrowed_ids.retain(|x| *x != id);
            Ok(())
        } else {
            Err(format!("No item found with id: {}", id))
        }
    }

    #[allow(dead_code)]
    pub fn is_borrowed_list_valid(&self, cat: &Catalog) -> bool {
        for id in &self.borrowed_ids {
            if cat.get(&id).is_err() {
                return false;
            }
        }
        true
    }
}
