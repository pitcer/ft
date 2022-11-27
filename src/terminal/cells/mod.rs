use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

use crate::spatial::dimension::Dimensions;
use crate::spatial::point::Point;
use crate::spatial::CellsUnit;
use crate::terminal::cells::line::{Cell, Line, RendererAction};

pub mod line;

#[derive(Debug)]
pub struct Cells {
    size: Dimensions<CellsUnit>,
    lines: VecDeque<Line>,
    current_cell: Point<CellsUnit>,
}

impl Cells {
    pub fn new(size: Dimensions<CellsUnit>) -> Self {
        assert!(size.width() >= 1 && size.height() >= 1);

        let lines = VecDeque::from(vec![
            Line::new(size.width() as usize);
            size.height() as usize
        ]);
        let first_cell = Point::new(0, 0);
        Self {
            size,
            lines,
            current_cell: first_cell,
        }
    }

    pub fn push_character(&mut self, character: Option<char>) -> RendererAction {
        let cell_point = self.current_cell;

        let cell = self.cell_mut(cell_point);
        *cell.character_mut() = character;

        if self.current_cell.horizontal_distance() == self.size.width() - 1 {
            self.carriage_return();
            let action = self.new_line();
            if let Some(action) = action {
                return action;
            }
        } else {
            self.current_cell = self.current_cell.shifted(1, 0);
            log::trace!("New current cell: {:?}", self.current_cell);
        }
        RendererAction::RenderCell(cell_point)
    }

    pub fn carriage_return(&mut self) {
        let vertical_distance = self.current_cell.vertical_distance();
        self.current_cell = Point::new(0, vertical_distance);
        log::trace!("New current cell: {:?}", self.current_cell);
    }

    pub fn move_back(&mut self) {
        if self.current_cell.horizontal_distance() == 0 {
            let vertical_distance = self.current_cell.vertical_distance();
            if vertical_distance > 0 {
                self.current_cell = Point::new(self.size.width() - 1, vertical_distance - 1);
                log::trace!("New current cell: {:?}", self.current_cell);
            }
        } else {
            self.current_cell = self.current_cell.shifted(-1, 0);
            log::trace!("New current cell: {:?}", self.current_cell);
        }
    }

    pub fn move_up(&mut self) {
        if self.current_cell.vertical_distance() > 0 {
            self.current_cell = self.current_cell.shifted(0, -1);
            log::trace!("New current cell: {:?}", self.current_cell);
        }
    }

    pub fn new_line(&mut self) -> Option<RendererAction> {
        if self.current_cell.vertical_distance() == self.size.height() - 1 {
            self.lines.rotate_left(1);
            let line = self.lines.back_mut().expect("Height is at least 1");
            line.clear();
            log::trace!("Lines rotated");
            Some(RendererAction::RenderAll)
        } else {
            self.current_cell = self.current_cell.shifted(0, 1);
            log::trace!("New current cell: {:?}", self.current_cell);
            None
        }
    }

    pub fn iter(&self) -> Iter<Line> {
        self.lines.iter()
    }

    pub fn clear(&mut self) {
        for line in &mut self.lines {
            line.clear();
        }
        self.current_cell = Point::new(0, 0);
    }

    fn cell_mut(&mut self, cell: Point<CellsUnit>) -> &mut Cell {
        let line_index = cell.vertical_distance() as usize;
        let line = &mut self.lines[line_index];
        let cell_index = cell.horizontal_distance() as usize;
        line.cell_mut(cell_index)
    }

    pub fn current_cell(&self) -> Point<CellsUnit> {
        self.current_cell
    }
}
