#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rgba([u8; 4]);

impl Rgba {
    pub fn blend(&self, background: Rgb) -> Rgb {
        Rgb([
            Self::mix(self.0[0] as u32, background.0[0] as u32, self.0[3] as u32),
            Self::mix(self.0[1] as u32, background.0[0] as u32, self.0[3] as u32),
            Self::mix(self.0[2] as u32, background.0[0] as u32, self.0[3] as u32),
        ])
    }

    fn mix(foreground: u32, background: u32, foreground_alpha: u32) -> u8 {
        let result = (foreground * foreground_alpha + background * (255 - foreground_alpha)) / 255;
        debug_assert!(result <= 255);
        result as u8
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rgb([u8; 3]);

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue])
    }

    pub fn with_alpha(&self, alpha: Alpha) -> Rgba {
        Rgba([self.0[0], self.0[1], self.0[2], alpha.0])
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Alpha(u8);

impl Alpha {
    pub fn new(alpha: u8) -> Self {
        Self(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend() {
        let background = Rgb::new(32, 32, 32);
        let foreground = Rgb::new(249, 250, 244);

        let color = foreground.with_alpha(Alpha(255));
        let color = color.blend(background);
        assert_eq!(foreground, color);

        let color = foreground.with_alpha(Alpha(0));
        let color = color.blend(background);
        assert_eq!(background, color);
    }
}
