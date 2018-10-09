struct Foo { opt: Option<String>, }
impl Foo {
  fn fun(&mut self) {
    if let Some(ref val) = &self.opt { // immutable borrow occurs here
      println!("{}", val);
      self.gun(); // mutable borrow occurs here
    } // immutable borrow ends here
  }
  fn gun(&mut self) { println!("Norbert"); }
}

fn main() {
  let mut foo = Foo { opt: Some(String::from("Norbi")), };
  foo.fun();
}
