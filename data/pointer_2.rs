pub fn main() {
    let mut s = String::from("Denny");
    let sp = &mut s as *mut String;
    let sm = unsafe { &mut *sp };
    let sm2 = unsafe { &mut *sp };
    sm.reserve(42);
    sm2.reserve(42);
}
