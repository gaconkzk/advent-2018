use std::fs::{self};
use std::io::{Result};

fn main() -> Result<()> {
  // read input file
  let input = fs::read_to_string("input/d03-sample.txt")?;
  p1(&input)?;
  let input = fs::read_to_string("input/d03.txt")?;
  p1(&input)?;

  Ok(())
}

#[derive(Debug)]
struct Claims {
  top: u16,
  left: u16,
  width: u16,
  height: u16,
}

impl Claims {
  fn new(line: &str) -> Claims {
    let parts:Vec<&str> = line.split(' ').collect();
    let pos_parts:Vec<&str> = parts[2]
        .split(':')
        .collect();
    let pos_part: Vec<&str> = pos_parts[0].split(',').collect();
    let position:Vec<u16> = pos_part.iter()
        .map(|p| String::from(*p))
        .map(|p| p.parse().unwrap())
        .collect();

    let size_parts:Vec<&str> = parts[3].split('x').collect();
    let size:Vec<u16> = size_parts.iter()
        .map(|s| String::from(*s))
        .map(|s| s.parse().unwrap())
        .collect();

    Claims {
      left: position[0],
      top: position[1],
      width: size[0],
      height: size[1],
    }
  }
}

fn fill_fab(claim: &Claims, fab: &mut [[u16; 2000]; 2000]) {
  for x in claim.left..claim.left+claim.width {
    for y in claim.top..claim.top+claim.height {
      fab[x as usize][y as usize] += 1;
    }
  }
}

fn p1(input: &str) -> Result<()> {
  let mut fab= [[0; 2000]; 2000];

  input.lines()
      .map(|l| Claims::new(l) )
      .for_each(|c| fill_fab(&c, &mut fab));

  let a:Vec<u16> = fab.iter()
      .flat_map(|arr| arr.iter())
      .cloned()
      .collect();
  let a:Vec<u16> = a.iter()
      .map(|a| *a)
      .filter(|f| *f > 1).collect();

  println!("p1: {:?}", a.len());

  Ok(())
}
