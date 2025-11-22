pub trait Item {
    fn get_id(&self) -> &str;

    fn get_title(&self) -> &str;

    fn days_allowed(&self) -> u8;
}

pub struct Book {
    id: String,
    title: String,
}

pub struct Dvd {
    id: String,
    title: String,
}

impl Book {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}

impl Dvd {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}

impl Item for Book {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn days_allowed(&self) -> u8 { 14 }
}

impl Item for Dvd {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn days_allowed(&self) -> u8 { 7 }
}
