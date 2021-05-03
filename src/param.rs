pub struct None;

impl ToString for None {
    fn to_string(&self) -> String {
        String::new()
    }
}

#[derive(Debug, Default)]
pub struct Accounts {
    pub count: Option<usize>,
    pub sort: Option<String>,
    pub start: Option<usize>,
}

impl ToString for Accounts {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if let Some(count) = self.count {
            s.push_str(&format!("count={}&", count));
        }
        if let Some(sort) = &self.sort {
            s.push_str(&format!("sort={}&", sort));
        }
        if let Some(start) = self.start {
            s.push_str(&format!("start={}&", start));
        }

        s
    }
}
