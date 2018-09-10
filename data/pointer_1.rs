pub fn main() {
    let s = String::from("Matze");
    let sp = &s as *const String;
    println!(
        "{}",
        unsafe { &*sp }.len()
    );
}
