use crate::{rgb::RGB, TMixedColor};

pub struct Color {
    pub rgb: RGB,
    pub decorator: Option<Box<dyn TMixedColor>>,
}

impl TMixedColor for Color {
    fn mix(&self) -> RGB {
        let mut result = self.rgb;
        if let Some(ref dec) = self.decorator {
            result = self.rgb.mix(&dec.mix())
        }
        return result;
    }
}
