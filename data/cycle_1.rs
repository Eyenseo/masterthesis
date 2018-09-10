struct Node<'a> {
    pub next: &'a Node<'a>,
}
pub fn main() {
    let a = Node {};
    // missing field `next` in
    //   initializer of `Node<'_>`
}
