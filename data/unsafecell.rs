use std::cell::UnsafeCell;

struct Node<'a> {
  pub s: &'static str,
  pub next: UnsafeCell<Vec<&'a Node<'a>>>,
}
fn main() {
  let mut arena: Vec<Node> = vec![
    Node { s: "Nadine", next: UnsafeCell::default() },
    Node { s: "Andy", next: UnsafeCell::default() },
  ];
  arena[0].s = "Hubi";
  unsafe { &mut *{ &arena[0] }.next.get() }.push(&arena[1]);
  unsafe { &mut *{ &arena[1] }.next.get() }.push(&arena[0]);
  arena[1].s = "Quack"; // cannot borrow `arena` as mutable because it is
                        //   also borrowed as immutable
}
