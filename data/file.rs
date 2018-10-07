use std::io::{Read, Seek, SeekFrom};

fn main() {
  let args = std::env::args().nth(0).unwrap();
  let file = std::fs::File::open(args).unwrap();

  let x = (&file).bytes().next();
  println!("{:?}", x); // Some(Ok(127))
  let x = (&file).bytes().next();
  println!("{:?}", x); // Some(Ok(69))
  // Tell position
  file.seek(SeekFrom::Current(0)); // cannot borrow mutably
}
