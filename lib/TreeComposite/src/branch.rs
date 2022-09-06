use crate::{bud::BudBuilder, leaf::LeafBuilder, TTreeContainer, TTreePart};

pub struct Branch {
    num: u32,
    ends: Vec<Box<dyn TTreePart>>,
}

impl Branch {
    pub fn new(num: u32) -> Self {
        Self {
            num,
            ends: Vec::new(),
        }
    }
}

impl TTreePart for Branch {
    fn show(&self) -> String {
        let mut res = format!("Branch{}:", self.num);
        for i in self.ends.iter() {
            let tmp = &format!("\n{}", i.show());
            res += tmp;
        }
        res.replace("\n", "\n\t")
    }

    fn get_box(&self) -> Box<dyn TTreePart> {
        Box::new(self.clone())
    }
}

impl TTreeContainer for Branch {
    fn add(&mut self, part: impl TTreePart) {
        let member = part.get_box();
        self.ends.push(member);
    }
}

impl Clone for Branch {
    fn clone(&self) -> Self {
        let mut cb = Branch::new(self.num + 1);
        for i in self.ends.iter() {
            let tmp = i.as_ref();
            let member = tmp.get_box();
            cb.ends.push(member);
        }
        cb
    }

    fn clone_from(&mut self, source: &Self) {
        self.ends = Vec::new();
        for i in source.ends.iter() {
            let tmp = i.as_ref();
            let member = tmp.get_box();
            self.ends.push(member);
        }
    }
}

pub struct BranchAlternateBuilder {
    num: u32,
    buds: Option<BudBuilder>,
    leafs: Option<LeafBuilder>,
    counts: (u32, u32),
}

impl BranchAlternateBuilder {
    pub fn build(&mut self) -> Branch {
        let mut res = Branch::new(self.num);
        self.num += 1;

        let (mut budcnt, mut leafcnt) = self.counts;

        match (&mut self.buds, &mut self.leafs) {
            (None, None) => { /* nothing */ }
            (None, Some(lbuilder)) => {
                for _ in 0..leafcnt {
                    let leaf = lbuilder.build();
                    res.add(leaf);
                }
            }
            (Some(bbuilder), None) => {
                for _ in 0..budcnt {
                    let bud = bbuilder.build();
                    res.add(bud);
                }
            }
            (Some(bbuilder), Some(lbuilder)) => {
                while 0 < budcnt || 0 < leafcnt {
                    if 0 != budcnt {
                        let bud = bbuilder.build();
                        res.add(bud);
                        budcnt -= 1;
                    }
                    if 0 != leafcnt {
                        let leaf = lbuilder.build();
                        res.add(leaf);
                        leafcnt -= 1;
                    }
                }
            }
        }

        res
    }
}

impl Branch {
    pub fn builderAlternate(
        start_num: u32,
        bud_builder: Option<BudBuilder>,
        leaf_builder: Option<LeafBuilder>,
        counts: (u32, u32),
    ) -> BranchAlternateBuilder {
        BranchAlternateBuilder {
            num: start_num,
            buds: bud_builder,
            leafs: leaf_builder,
            counts,
        }
    }
}
