struct Proper;
trait Object {}
impl Object for Proper {}

#[repr(C)]
struct TraitObject {
  pub data: *const (),
  pub vtable: *const (),
}
fn main() {
  let vtable = unsafe {
    std::mem::transmute::<_, TraitObject>(
      std::ptr::null::<Proper>() as *const Object
    ).vtable
  };
}
