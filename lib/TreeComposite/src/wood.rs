use crate::{TTreeContainer, TTreePart};

pub struct Wood {
    branches: Vec<Box<dyn TTreePart>>,
}
impl Wood {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
        }
    }
}

impl TTreePart for Wood {
    fn show(&self) -> String {
        let mut res = format!("Wood:");
        for i in self.branches.iter() {
            let tmp = &format!("\n{}", i.show());
            res += tmp;
        }
        res.replace("\n", "\n\t")
    }

    fn get_box(&self) -> Box<dyn TTreePart> {
        todo!()
    }
}

impl TTreeContainer for Wood {
    fn add(&mut self, part: impl TTreePart) {
        let member = part.get_box();
        self.branches.push(member);
    }
}
