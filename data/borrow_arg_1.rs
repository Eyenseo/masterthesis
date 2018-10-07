use std::cell::RefCell;
use std::collections::HashMap;

struct F {}
impl F {
  fn fun(&mut self) -> i8 { 42 }
}

fn gun(f: &RefCell<F>)
  -> HashMap<i8, i8>
{
  let mut map = HashMap::new();
  map.insert(
    f.borrow_mut().fun(),
    f.borrow_mut().fun() // BOOM
  );
  map
}

fn main() {
  let f = RefCell::new(F {});
  fun(&f);
}
