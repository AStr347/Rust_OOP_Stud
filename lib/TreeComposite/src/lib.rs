use std::fmt;

pub trait TTreePart {
    fn show(&self) -> String;
    fn get_box(&self) -> Box<dyn TTreePart>;
}

impl fmt::Display for dyn TTreePart {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let part_data = &format!("{}", self.show());
        fmt.write_str(part_data)?;
        Ok(())
    }
}

pub trait TTreeContainer: TTreePart {
    fn add(&mut self, part: impl TTreePart);
}

pub mod branch;
pub mod bud;
pub mod leaf;
pub mod wood;
