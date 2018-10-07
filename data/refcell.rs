use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Foo {
  v: Vec<Rc<String>>,
}
impl Foo {
  fn new() -> Foo {
    Foo { v: Vec::new() }
  }
  fn fun(&mut self, s: &str) -> Rc<String> {
    self.v.push(Rc::new(s.to_owned()));
    self.v.last().unwrap().clone()
  }
}

struct Bar {
  foo: Rc<RefCell<Foo>>,
}
fn main() {
  let mut v: Vec<Weak<String>> = Vec::new();
  let bar = Bar {
    foo: Rc::new(RefCell::new(Foo::new())),
  };
  v.push(bar.foo.borrow_mut().fun("Foo").downgrade());
}
