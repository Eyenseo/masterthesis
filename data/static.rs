struct Foo;
impl Foo { fn num() -> usize { 3 } }
trait Bar {
    fn num() -> usize
        where Self: Sized,
    { 1 }
}
trait Baz {
    fn num() -> usize
        where Self: Sized,
    { 2 }
}

trait IDs { fn id() -> usize; }
impl IDs for Bar {
    fn id() -> usize { 4 }
}
impl IDs for Baz {
    fn id() -> usize { 5 }
}
impl IDs for Foo {
    fn id() -> usize { 6 }
}
impl Bar for Foo {}
impl Baz for Foo {}

pub fn main() {
    // println!("{}", Bar::num()); // type annotations required:
                                   // cannot resolve `_: Bar`

    println!("{}", <Foo as Bar>::num());
    println!("{}", <Foo as Baz>::num());
    println!("{}", Foo::num());

    println!("{}", Bar::id());
    println!("{}", Baz::id());
    println!("{}", Foo::id());
}
