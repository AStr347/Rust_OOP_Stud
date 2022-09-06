use crate::TTreePart;

#[derive(Clone)]
pub struct Bud {
    num: u32,
}

pub struct BudBuilder {
    num: u32,
}

impl BudBuilder {
    pub fn build(&mut self) -> Bud {
        let res = Bud { num: self.num };
        self.num += 1;
        res
    }
}

impl Bud {
    pub fn builder(start_num: u32) -> BudBuilder {
        BudBuilder { num: start_num }
    }
}

impl TTreePart for Bud {
    fn show(&self) -> String {
        format!("Bud{}", self.num)
    }

    fn get_box(&self) -> Box<dyn TTreePart> {
        Box::new(self.clone())
    }
}
