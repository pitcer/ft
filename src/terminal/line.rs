#[derive(Debug, Clone)]
pub struct Line {
    text: String,
}

impl Line {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
