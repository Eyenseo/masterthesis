trait A { fn fun(&self); }
trait B: A { fn gun(&self); }

struct Foo;

impl B for Foo { fn gun(&self) {} }
impl A for Foo { fn fun(&self) {} }

fn main() {
  let f = Foo;
  let b: &B = &f;
  b.fun();
  let a: &A = b; // expected trait `A`
                 //   found trait `B`
}
