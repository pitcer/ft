use std::marker::PhantomData;

use nix::libc;
use nix::pty::Winsize;

use crate::spatial::point::Point;
use crate::spatial::{CellsUnit, PixelsUnit};

#[derive(Debug, Copy, Clone)]
pub struct Dimensions<Unit> {
    width: u32,
    height: u32,
    unit: PhantomData<Unit>,
}

impl<Unit> Dimensions<Unit> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            unit: PhantomData,
        }
    }

    pub fn vector_index(&self, point: Point<Unit>) -> usize {
        self.width as usize * point.vertical_distance() as usize
            + point.horizontal_distance() as usize
    }

    pub fn contains(&self, point: Point<Unit>) -> bool {
        point.horizontal_distance() <= self.width && point.vertical_distance() <= self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Dimensions<PixelsUnit> {
    pub fn fit_cells(&self, cell_size: Dimensions<PixelsUnit>) -> Dimensions<CellsUnit> {
        let width = self.width / cell_size.width;
        let height = self.height / cell_size.height;
        Dimensions::new(width, height)
    }
}

impl From<Dimensions<CellsUnit>> for Winsize {
    fn from(size: Dimensions<CellsUnit>) -> Self {
        Winsize {
            ws_row: size.height() as libc::c_ushort,
            ws_col: size.width() as libc::c_ushort,
            ws_xpixel: 0, // unused
            ws_ypixel: 0, // unused
        }
    }
}
