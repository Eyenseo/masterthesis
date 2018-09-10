struct Foo {
  s: &String, // missing lifetime
              // specifier
}

pub fn main() {
  let foo = {
    let s = String::from("Maria");
    Foo { s: &s }
  };
}
