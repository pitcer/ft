use std::slice;

use anyhow::Result;
use framebuffer::Framebuffer;

use crate::dimension::{Dimensions, PixelsUnit};
use crate::display::pixel::DisplayPixel;
use crate::point::Point;

pub mod pixel;

#[derive(Debug)]
pub struct Display {
    framebuffer: Framebuffer,
}

impl Display {
    pub fn new(framebuffer_device_path: &str) -> Result<Self> {
        let framebuffer = Framebuffer::new(framebuffer_device_path)?;

        // Only 4 bytes per pixel are supported.
        assert_eq!(32, framebuffer.var_screen_info.bits_per_pixel);
        // Only BGR0 pixel format is supported.
        assert_eq!(16, framebuffer.var_screen_info.red.offset);
        assert_eq!(8, framebuffer.var_screen_info.green.offset);
        assert_eq!(0, framebuffer.var_screen_info.blue.offset);
        assert_eq!(0, framebuffer.var_screen_info.transp.length);

        Ok(Self { framebuffer })
    }

    pub fn pixel_mut(&mut self, pixel: Point<PixelsUnit>) -> DisplayPixel {
        let size = self.size();
        debug_assert!(size.contains(pixel));

        let index = size.vector_index(pixel);
        let pixel = &mut self.pixel_chunks_mut()[index];
        DisplayPixel::from_frame_chunk(pixel)
    }

    pub fn pixel_chunks_mut(&mut self) -> &mut [[u8; 4]] {
        let frame = &mut self.framebuffer.frame;

        // SAFETY: Frame length is divisible by 4 because of precondition in constructor.
        // Then frame length is 4 * new_len, so its safe to cast it as new_len chunks of 4.
        unsafe {
            let new_len = frame.len() / 4;
            slice::from_raw_parts_mut(frame.as_mut_ptr().cast(), new_len)
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.frame.fill(0);
    }

    pub fn size(&self) -> Dimensions<PixelsUnit> {
        let width = self.framebuffer.var_screen_info.xres;
        let height = self.framebuffer.var_screen_info.yres;
        Dimensions::new(width, height)
    }
}
