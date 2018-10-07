fn main() {
  let s = String::from("Matze");
  let ps = &s as *const String;
  println!(
    "{}",
    unsafe { &*ps }.len()
  );
}
