#[derive(Debug, Clone)]
pub struct Line {
    text: Option<String>,
}

impl Line {
    pub fn new() -> Self {
        let text = None;
        Self { text }
    }
}
