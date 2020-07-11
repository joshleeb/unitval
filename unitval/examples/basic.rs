use unitval::{AsUnitVal, FromUnitVal, UnitVal};

#[derive(Debug, Clone, Copy, UnitVal)]
enum Foo {
    #[unitval = "A"]
    VarA,
}

fn main() {
    println!("{:?}", Foo::VarA.as_unitval()); // A
    println!("{:?}", Foo::from_unitval("A")); // Ok(Foo::VarA)
}
