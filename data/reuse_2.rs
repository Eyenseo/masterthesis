trait A { fn fun(&self); }
trait Accessor { fn s(&self) -> &'static str; }
struct Foo { s: &'static str, }

impl Accessor for Foo {
  fn s(&self) -> &'static str {
    self.s
  }
}
impl<T> A for T
where
  T: Accessor,
{
  fn fun(&self) {
    println!("{}", self.s());
    // no field `s` on type `&T`
  }
}
