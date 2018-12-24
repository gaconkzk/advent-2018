use std::fs::{self};
use std::io::{Result};
use std::collections::BTreeSet;

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
  let mut btree_set = BTreeSet::new();
  btree_set.insert(0);

  let mut fr2:i32 = -1;
  let mut i = 0;
  while fr2 < 0 {
    for line in input.lines() {
      let change:i32 = line.parse().unwrap();
      e += change;
      if btree_set.contains(&e) {
        fr2 = e;
        break;
      }
      btree_set.insert(e);
    }
    println!("i: {}, e: {}", i, e);
    i = i+1;
  }
  

  println!("p2: {:?}", e);

  Ok(())
}
