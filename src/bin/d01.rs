use std::fs::{self};
use std::io::{Result};
use std::vec::Vec;

fn main() -> Result<()> {
//   // read input file
  let input = fs::read_to_string("input/d01.txt")?;
  p1(&input)?;
  p2(&input)?;

  Ok(())
}

fn p1(input: &str) -> Result<()> {
  let mut e = 0;
  for line in input.lines() {
    let change:i32 = line.parse().unwrap();
    e += change;
  }

  println!("p1: {:?}", e);

  Ok(())
}

fn p2(input: &str) -> Result<()> {
  let mut e = 0;
  let mut vec = Vec::new();
  vec.push(0);

  let mut fr2:i32 = -1;
  let mut i = 0;
  while fr2 < 0 {
    for line in input.lines() {
      let change:i32 = line.parse().unwrap();
      e += change;
      if vec.contains(&e) {
        fr2 = e;
        break;
      }
      vec.push(e);
    }
    i = i+1;
  }
  

  println!("p2: {:?}", e);

  Ok(())
}
