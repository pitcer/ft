use crate::rgb::Rgb;

#[derive(Debug)]
pub struct DisplayPixel<'a>(&'a mut [u8; 4]);

impl<'a> DisplayPixel<'a> {
    pub(super) fn from_frame_chunk(chunk: &'a mut [u8; 4]) -> Self {
        Self(chunk)
    }

    pub fn set_rgb(&mut self, rgb: Rgb) {
        // Layout is guaranteed by precondition in Display, which is the only place
        // where this struct can be constructed.
        self.0[0] = rgb.blue();
        self.0[1] = rgb.green();
        self.0[2] = rgb.red();
    }
}
