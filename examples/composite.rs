use TreeComposite::{branch::*, bud::*, leaf::*, wood::*, *};

fn main() {
    let mut wood = Wood::new();

    let mut branch0_builder =
        Branch::builderAlternate(0, Some(Bud::builder(0)), Some(Leaf::builder(100)), (4, 8));

    let mut branch1_builder = Branch::builderAlternate(
        1000,
        Some(Bud::builder(20)),
        Some(Leaf::builder(50)),
        (8, 4),
    );

    let branch0 = branch0_builder.build();
    let branch1 = branch1_builder.build();
    let branch2 = branch0_builder.build();
    let branch3 = branch1_builder.build();

    wood.add(branch1);
    wood.add(branch0);
    wood.add(branch2);
    wood.add(branch3);

    let dynwood: &dyn TTreePart = &wood;
    println!("{}", dynwood);
}
