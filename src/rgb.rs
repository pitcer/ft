#[derive(Debug)]
pub struct Rgb([u8; 3]);

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue])
    }

    pub fn new_gray(gray: u8) -> Self {
        Self([gray, gray, gray])
    }

    pub fn red(&self) -> u8 {
        self.0[0]
    }

    pub fn green(&self) -> u8 {
        self.0[1]
    }

    pub fn blue(&self) -> u8 {
        self.0[2]
    }
}
