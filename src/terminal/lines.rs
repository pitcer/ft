use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Lines {
    lines: VecDeque<Line>,
    line_count: usize,
    current_lines: usize,
}

impl Lines {
    pub fn new(line_count: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(line_count),
            line_count,
            current_lines: 0,
        }
    }

    pub fn push_line(&mut self, line: String) {
        if self.current_lines == self.line_count {
            self.lines.pop_front();
        } else {
            self.current_lines += 1;
        }
        self.lines.push_back(Line::new(line));
    }

    pub fn iter(&self) -> Iter<Line> {
        self.lines.iter()
    }
}

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
