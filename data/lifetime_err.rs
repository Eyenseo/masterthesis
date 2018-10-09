struct Foo {
  s: &String, // missing lifetime
}             //   specifier
fn main() {
  let foo = {
    let s = String::from("Maria");
    Foo { s: &s } // No semicolon
  };              //   returns
}
