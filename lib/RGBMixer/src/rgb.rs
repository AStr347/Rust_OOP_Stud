#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

impl RGB {
    pub fn mix(&self, m: &Self) -> Self {
        let r = (self.r + m.r);
        let g = (self.g + m.g);
        let b = (self.b + m.b);

        let new_r = if (r < 0) {
            0
        } else {
            if (r > 255) {
                255
            } else {
                r
            }
        };
        let new_g = if (g < 0) {
            0
        } else {
            if (g > 255) {
                255
            } else {
                g
            }
        };
        let new_b = if (b < 0) {
            0
        } else {
            if (b > 255) {
                255
            } else {
                b
            }
        };

        Self {
            r: new_r,
            g: new_g,
            b: new_b,
        }
    }
}
