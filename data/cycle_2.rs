struct Node<'a> {
    pub next: Option<&'a  Node<'a>>,
}
pub fn main() {
    let mut a = Node { next: None };
    let mut b = Node {
        next: Some(&a)
    };
    a.next = Some(&b);
    // `b` does not live long enough
    // cannot assign to `a.next`
    //   because it is borrowed
}
