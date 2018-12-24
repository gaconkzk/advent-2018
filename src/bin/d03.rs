use std::fs::{self};
use std::io::{Result};

fn main() -> Result<()> {
  // read input file
  let input = fs::read_to_string("input/d03-sample.txt")?;
  p1(&input)?;
  let input = fs::read_to_string("input/d03.txt")?;
  p1(&input)?;

  p2(&input)?;

  Ok(())
}

struct Claims {
  id: u16,
  top: u16,
  left: u16,
  width: u16,
  height: u16,
}

impl Claims {
  fn new(line: &str) -> Claims {
    let parts:Vec<&str> = line.split(' ').collect();
    let id:u16 = parts[0][1..].parse().unwrap();
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
      id,
      left: position[0],
      top: position[1],
      width: size[0],
      height: size[1],
    }
  }
}

const F_SIZE:usize = 1000;
type Fabric = [[usize;F_SIZE]; F_SIZE];

fn fill_fab(claim: &Claims, fab: &mut Fabric) {
  for x in claim.left..claim.left+claim.width {
    for y in claim.top..claim.top+claim.height {
      fab[x as usize][y as usize] += 1;
    }
  }
}

fn cool_claim(c: &Claims, fab: &Fabric) -> Option<u16> {
  for x in c.left..c.left+c.width {
    for y in c.top..c.top + c.height {
      if fab[x as usize][y as usize] > 1 {
        // must use return statement
        return None
      }
    }
  }
  // must use return statement
  return Some(c.id)
}

fn p1(input: &str) -> Result<()> {
  let mut fab= [[0; F_SIZE]; F_SIZE];

  input.lines()
      .map(|l| Claims::new(l) )
      .for_each(|c| fill_fab(&c, &mut fab));

  let a = fab.iter()
      .flat_map(|arr| arr.iter())
      .map(|a| *a)
      .filter(|f| *f > 1)
      .collect::<Vec<usize>>()
      .len();

  println!("p1: {:?}", a);

  Ok(())
}

fn p2(input: &str) -> Result<()> {
  let mut fab= [[0; F_SIZE]; F_SIZE];

  let claims:Vec<Claims> = input.lines()
      .map(|l| Claims::new(l) )
      .collect();

  claims.iter().for_each(|c| fill_fab(&c, &mut fab));
  let id = claims.iter().find_map(|c| cool_claim(c, &fab)).unwrap();

  println!("p2: {:?}", id);

  Ok(())
}
