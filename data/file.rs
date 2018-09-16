use std::io::{Read, Seek, SeekFrom};

fn main() {
    let p = std::env::args().nth(0).unwrap();
    let f = std::fs::File::open(p).unwrap();

    let b = (&f).bytes().next();
    println!("{:?}", b); // Some(Ok(127))
    let b = (&f).bytes().next();
    println!("{:?}", b); // Some(Ok(69))
    // Tell position
    f.seek(SeekFrom::Current(0)); // cannot borrow mutably
}
