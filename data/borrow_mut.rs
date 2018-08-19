pub fn main() {
    let mut string = String::from("Hello");
    let foo = &string;
    println!("{}", string.capacity());
    println!("{}", foo.capacity());
    foo.reserve(10); // ERROR cannot borrow immutable borrowed content `*foo`
                     // as mutable
    string.reserve(10); // ERROR cannot borrow `s` as mutable because it is
                        // also borrowed as immutable
}
