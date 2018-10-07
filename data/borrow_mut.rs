fn main() {
  let mut s = String::from("Hello");
  let rs = &s;
  println!("{}", rs.capacity());
  println!("{}", s.capacity());
  rs.reserve(10); // cannot borrow immutable borrowed content `*rs` as
           // mutable
  s.reserve(10); // cannot borrow `s` as mutable because it is also
           // borrowed as immutable
}
