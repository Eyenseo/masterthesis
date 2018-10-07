struct Node<'a> {
  pub next: &'a Node<'a>,
}

fn main() {
  let x = Node {};
  // missing field `next`
  //   in initializer of
  //   `Node<'_>`
}
