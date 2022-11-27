use crate::spatial::point::Point;
use crate::spatial::CellsUnit;

#[derive(Debug, Clone)]
pub struct Line {
    cells: Vec<Cell>,
}

impl Line {
    pub fn new(length: usize) -> Self {
        let cells = vec![Cell::new(); length];
        Self { cells }
    }

    pub fn iter(&self) -> std::slice::Iter<Cell> {
        self.cells.iter()
    }

    pub fn cell_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.character = None;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    character: Option<char>,
}

impl Cell {
    pub fn new() -> Self {
        Self { character: None }
    }

    pub fn character_mut(&mut self) -> &mut Option<char> {
        &mut self.character
    }

    pub fn character(&self) -> Option<char> {
        self.character
    }
}

pub enum RendererAction {
    RenderAll,
    RenderCell(Point<CellsUnit>),
}
