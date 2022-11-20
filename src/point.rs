use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Point<U> {
    horizontal_distance: u32,
    vertical_distance: u32,
    unit: PhantomData<U>
}

impl<U> Point<U> {
    pub fn new(horizontal_distance: u32, vertical_distance: u32) -> Self {
        Self {
            horizontal_distance,
            vertical_distance,
            unit: PhantomData
        }
    }

    pub fn horizontal_distance(&self) -> u32 {
        self.horizontal_distance
    }

    pub fn vertical_distance(&self) -> u32 {
        self.vertical_distance
    }
}
