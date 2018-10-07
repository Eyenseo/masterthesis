struct Node<'a> {
  pub next: Option<&'a  Node<'a>>,
}
fn main() {
  let mut x = Node { next: None };
  let mut y = Node { next: Some(&x) };
  x.next = Some(&y);
  // `y` does not live long enough
  // cannot assign to `x.next` because
  //   it is borrowed
}
