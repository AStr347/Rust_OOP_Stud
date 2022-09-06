use RGBMixer::{color::Color, rgb::RGB, *};

fn main() {
    let Red = Color {
        rgb: RGB { r: 255, g: 0, b: 0 },
        decorator: None,
    };
    let SomeGreenWithRed = Color {
        rgb: RGB { r: 0, g: 96, b: 0 },
        decorator: Some(Box::new(Red)),
    };
    let Black = Color {
        rgb: RGB { r: 0, g: 0, b: 0 },
        decorator: Some(Box::new(SomeGreenWithRed)),
    };

    println!("{:?}", Black.mix());
}
