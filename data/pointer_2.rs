fn main() {
  let mut s = String::from("Denny");
  let ps = &mut s as *mut String;
  let rms1 = unsafe { &mut *ps };
  let rms2 = unsafe { &mut *ps };
  rms2.reserve(42);
  rms1.reserve(42);
}
