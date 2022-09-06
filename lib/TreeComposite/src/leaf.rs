use crate::TTreePart;

#[derive(Clone)]
pub struct Leaf {
    num: u32,
}

pub struct LeafBuilder {
    num: u32,
}

impl LeafBuilder {
    pub fn build(&mut self) -> Leaf {
        let res = Leaf { num: self.num };
        self.num += 1;
        res
    }
}

impl Leaf {
    pub fn builder(start_num: u32) -> LeafBuilder {
        LeafBuilder { num: start_num }
    }
}

impl TTreePart for Leaf {
    fn show(&self) -> String {
        format!("Leaf{}", self.num)
    }

    fn get_box(&self) -> Box<dyn TTreePart> {
        Box::new(self.clone())
    }
}
