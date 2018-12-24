use std::fs::{self};
use std::io::{Result};
use std::collections::BTreeMap;

fn main() -> Result<()> {
  // read input file
  let input = fs::read_to_string("input/d02-sample.txt")?;
  p1(&input)?;
  let input = fs::read_to_string("input/d02.txt")?;
  p1(&input)?;

  let input = fs::read_to_string("input/d02-2-sample.txt")?;
  p2(&input)?;
  let input = fs::read_to_string("input/d02.txt")?;
  p2(&input)?;

  Ok(())
}

fn compare(a: &str, b: &str) -> bool {
  if a.len() != b.len() {
    return false
  }

  let t_size = a.len();
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();
  let mut t: i32 = 0;

  for i in 0..t_size {
    if a_bytes[i] != b_bytes[i] {
      if t > 0 {
        return false
      }
      t = 1;
    }
  }

  return true
}

fn common(a: &str, b: &str) -> String {
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();

  let mut result = String::new();
  let t_size = a.len();
  for i in 0..t_size {
    if a_bytes[i] != b_bytes[i] {
      continue;
    }
    result.push(a.chars().nth(i).unwrap());
  }

  result
}

fn p2(input: &str) -> Result<()> {
  let mchar: Vec<_> = input.lines().map(|l| l.chars()).collect();
  for i in 0..mchar.len() {
    for j in i+1..mchar.len() {
      if compare(mchar[i].as_str(), mchar[j].as_str()) {
        println!("p2: {} vs {}", mchar[i].as_str(), mchar[j].as_str());
        println!("p2: {}", common(mchar[i].as_str(), &mchar[j].as_str()));
      }
    }
  }
  Ok(())
}

fn p1(input: &str) -> Result<()> {
  let mut sc1 = 0;
  let mut sc2 = 0;
  for line in input.lines() {
    let mut map = BTreeMap::new();
    let mut count2 = 0;
    let mut count3 = 0;
    for c in line.chars() {
      *map.entry(c).or_insert(0) += 1;
    }
    let d: Vec<_> = map.iter().map(|(_, v)| v).filter(|&v| (*v) > 1).collect();
    if d.contains(&&2) && d.contains(&&3) {
      count2 = 1;
      count3 = 1;
    } else {
      if d.contains(&&2) {
        count2 = 1;
      } else if d.contains(&&3) {
        count3 = 1;
      }
    }
    sc1 += count2;
    sc2 += count3;
  }

  println!("p1: {:?}", sc1 * sc2);

  Ok(())
}
