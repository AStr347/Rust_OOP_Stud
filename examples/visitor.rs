struct M {
    pub value: u32,
}
struct SM {
    pub value: u32,
}
struct MM {
    pub value: u32,
}

trait TVisitor {
    fn visit_m(&self, arg: &mut M);
    fn visit_sm(&self, arg: &mut SM);
    fn visit_mm(&self, arg: &mut MM);
}

struct SumMmVisitor {
    pub value: u32,
}

impl TVisitor for SumMmVisitor {
    fn visit_m(&self, arg: &mut M) {
        let mms = self.value;
        arg.value += mms / 1000;
    }

    fn visit_sm(&self, arg: &mut SM) {
        let mms = self.value;
        arg.value += mms / 10;
    }

    fn visit_mm(&self, arg: &mut MM) {
        let mms = self.value;
        arg.value += mms;
    }
}

fn main() {
    let mut ms = M { value: 101 };
    let mut sms = SM { value: 1010 };
    let mut mms = MM { value: 10100 };

    let visitor = SumMmVisitor { value: 10000 };

    visitor.visit_m(&mut ms);
    visitor.visit_sm(&mut sms);
    visitor.visit_mm(&mut mms);

    println!("m  : {}", ms.value);
    println!("sm : {}", sms.value);
    println!("mm : {}", mms.value);
}
