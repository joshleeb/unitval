use unitval::{AsUnitVal, FromUnitVal, UnitVal};

#[derive(Debug, Clone, Copy, UnitVal)]
enum Foo {
    #[unitval = "A"]
    VarA,
    #[unitval = "B"]
    VarB,
}

fn main() {
    println!("{:?}", Foo::VarA.as_unitval()); // A
    println!("{:?}", Foo::from_unitval("A")); // Ok(Foo::VarA)

    println!("{:?}", Foo::VarB.as_unitval()); // B
    println!("{:?}", Foo::from_unitval("B")); // Ok(Foo::VarB)
}
