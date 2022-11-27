use std::marker::PhantomData;
use std::ops::Add;

use crate::dimension::{CellsUnit, Dimensions, PixelsUnit};

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

    pub fn shifted(&self, horizontal_shift: u32, vertical_shift: u32) -> Point<Unit> {
        let horizontal_distance = self.horizontal_distance + horizontal_shift;
        let vertical_distance = self.vertical_distance + vertical_shift;
        Point::new(horizontal_distance, vertical_distance)
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

impl<Unit> Add<(u32, u32)> for Point<Unit> {
    type Output = Point<Unit>;

    fn add(self, rhs: (u32, u32)) -> Self::Output {
        self.shifted(rhs.0, rhs.1)
    }
}

impl Point<CellsUnit> {
    pub fn to_pixels(self, cell_size: Dimensions<PixelsUnit>) -> Point<PixelsUnit> {
        let horizontal_distance = self.horizontal_distance * cell_size.width();
        let vertical_distance = self.vertical_distance * cell_size.height();
        Point::new(horizontal_distance, vertical_distance)
    }
}
