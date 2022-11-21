use std::marker::PhantomData;

use crate::dimension::{Cells, Dimensions, Pixels};

#[derive(Debug, Copy, Clone)]
pub struct Point<Unit> {
    horizontal_distance: u32,
    vertical_distance: u32,
    unit: PhantomData<Unit>,
}

impl<Unit> Point<Unit> {
    pub fn new(horizontal_distance: u32, vertical_distance: u32) -> Self {
        Self {
            horizontal_distance,
            vertical_distance,
            unit: PhantomData,
        }
    }

    pub fn with_origin(&self, origin: Point<Unit>) -> Point<Unit> {
        let horizontal_distance = origin.horizontal_distance + self.horizontal_distance;
        let vertical_distance = origin.vertical_distance + self.vertical_distance;
        Point::new(horizontal_distance, vertical_distance)
    }

    pub fn horizontal_distance(&self) -> u32 {
        self.horizontal_distance
    }

    pub fn vertical_distance(&self) -> u32 {
        self.vertical_distance
    }
}

impl Point<Cells> {
    pub fn to_pixels(self, cell_size: Dimensions<Pixels>) -> Point<Pixels> {
        let horizontal_distance = self.horizontal_distance * cell_size.width();
        let vertical_distance = self.vertical_distance * cell_size.height();
        Point::new(horizontal_distance, vertical_distance)
    }
}
