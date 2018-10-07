trait A { fn fun(&self); }
trait B: A { fn gun(&self); }

struct Foo;

impl B for Foo { fn gun(&self) {} }
impl A for Foo { fn fun(&self) {} }

fn main() {
  let x = Foo;
  let rb: &B = &x;
  rb.fun();
  let ra: &A = rb; // expected
      // trait `A` found trait `B`
}
