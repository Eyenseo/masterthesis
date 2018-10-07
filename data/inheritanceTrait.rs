struct AProper {
  stefi: String,
}
struct BProper {
  stefi: String,
  dani: u32,
}

trait A {
  fn get_stefi(&self) -> &String;
  fn get_stefi_mut(&mut self) -> &mut String;
  fn set_stefi(&mut self, stefi: String);
}
trait B: A {
  fn get_dani(&self) -> u32;
  fn set_stefi(&mut self, stefi: u32);
}
