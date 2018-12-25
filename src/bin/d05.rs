use std::fs::{self};
use std::io::{Result};

fn main() -> Result<()> {
  // read input file
  let input = fs::read_to_string("input/d05-sample.txt")?;
  p1(&input)?;
  let input = fs::read_to_string("input/d05.txt")?;
  p1(&input)?;

  let input = fs::read_to_string("input/d05-sample.txt")?;
  p2(&input)?;
  let input = fs::read_to_string("input/d05.txt")?;
  p2(&input)?;

  Ok(())
}

trait OppoChar {
  fn is_opposite_with(&self, other: &char) -> bool;
}

impl OppoChar for Option<char> {
  fn is_opposite_with(&self, other: &char) -> bool {
    match self {
      Some(me) => {
        if other.is_lowercase() && me.is_uppercase() {
          return other.to_uppercase().last().unwrap() == *me
        }
        if other.is_uppercase() && me.is_lowercase() {
          return other.to_lowercase().last().unwrap() == *me
        }
        false
      },
      None => false
    }
  }
}

fn reacting(acc: &mut String, k: char) -> &mut String {
  let last_char = acc.chars().last();
  if last_char.is_opposite_with(&k) {
    acc.remove(acc.len()-1);
  } else {
    acc.push(k);
  }
  acc
}

fn p1(input: &str) -> Result<()> {
  let mut polymer = String::new();
  input.chars()
      .fold(&mut polymer, |acc, k| reacting(acc,k));

  println!("p1: {}", polymer.len());
  Ok(())
}

fn p2(input: &str) -> Result<()> {
  let min_len = (b'a'..=b'z')
      .map(|c| c as char)
      .map(|cca| {
        let mut polymer = String::new();
        let t_input = input.replace(cca, "").replace(cca.to_uppercase().last().unwrap(), "");
        t_input.chars()
            .fold(&mut polymer, |acc, k| reacting(acc, k));

        return polymer.len()
      })
      .min().unwrap();

  println!("p2: {}", min_len);
  Ok(())
}
