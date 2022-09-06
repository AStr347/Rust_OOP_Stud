pub mod rgb;

use rgb::RGB;
pub trait TMixedColor {
    fn mix(&self) -> RGB;
}

pub mod color;
