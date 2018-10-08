use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
  pub s: &'static str,
  pub next: Vec<Weak<RefCell<Node>>>,
}
fn main() {
  let x = Rc::new(RefCell::new(Node { s: "Lore", next: Vec::default() }));
  let y = Rc::new(RefCell::new(Node { s:"Steffi", next:Vec::default() }));
  y.borrow_mut().next.push(Rc::downgrade(&x));
  x.borrow_mut().next.push(Rc::downgrade(&y));
  y.borrow_mut().s = "Beni";
}
