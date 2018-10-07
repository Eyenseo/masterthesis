struct Foo {}
trait A { fn fun(&self); }
trait B: A {}

impl B for Foo {}
impl A for Foo { fn fun(&self) {} }

fn main() {
  let x = Foo {};
  let rb: &B = &x;
  rb.fun();
  let ra1: &A = rb; // expected trait `A`, found trait `B`
  let ra2: &A = rb as &A; // an `as` expression can only be used to convert
                          // between primitive types.
  let ra3: &A = &b;
}
