struct Foo {}
trait A { fn fun(&self); }
trait B: A {}

impl B for Foo {}
impl A for Foo { fn fun(&self) {} }

fn main() {
    let f = Foo {};
    let b: &B = &f;
    b.fun();
    let a: &A = b;
    // expected trait `A`,
    // found trait `B`
}
