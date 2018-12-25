use std::fs::{self};
use std::io::Result as IoResult;
use std::cmp::Ordering;
use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;

fn main() -> IoResult<()> {
  // read input file
  let input = fs::read_to_string("input/d04.txt")?;
  p1(&input)?;

  Ok(())
}

#[derive(Debug, Eq)]
struct TimedData {
  f_time: NaiveDateTime,
  minute: u32,
  action: GuardAction,
}

impl TimedData {
  fn new(line: &str) -> TimedData {
    let parts:Vec<&str> = line[1..].split("] ")
        .collect();

    let f_time = NaiveDateTime::parse_from_str(parts[0], "%Y-%m-%d %H:%M").unwrap();
    let minute = f_time.minute();
    let action = GuardAction::from(parts[1], &minute);
    TimedData {
      f_time,
      minute,
      action,
    }
  }
}

struct GuardHistory {
  sleep_time: [u8; 60],
  total_sleep_time: u32,
}

#[derive(Debug, Eq, PartialOrd, PartialEq)]
enum GuardAction {
  Shift(usize),
  Sleep(u32),
  Wakeup(u32),
  None,
}

impl GuardAction {
  fn from(action: &str, at: &u32) -> GuardAction {
    if action.chars().nth(0).unwrap() == 'G' {
      let id = action.chars().skip(7).take_while(|&c| c != ' ').collect::<String>().parse().unwrap();
      return GuardAction::Shift(id)
    } else if action.chars().nth(0).unwrap() == 'f' {
      return GuardAction::Sleep(*at)
    } else if action.chars().nth(0).unwrap() == 'w' {
      return GuardAction::Wakeup(*at)
    }

    GuardAction::None
  }
}

impl GuardHistory {
  fn new() -> GuardHistory {
    GuardHistory {
      sleep_time: [0; 60],
      total_sleep_time: 0,
    }
  }
}

impl PartialOrd for TimedData {
  fn partial_cmp(&self, other: &TimedData) -> Option<Ordering> {
    self.f_time.partial_cmp(&other.f_time)
  }
}

impl Ord for TimedData {
  fn cmp(&self, other: &TimedData) -> Ordering {
    self.f_time.cmp(&other.f_time)
  }
}

impl PartialEq for TimedData {
  fn eq(&self, other: &TimedData) -> bool {
    self.f_time == other.f_time
  }
}

fn p1(input: &str) -> IoResult<()> {
  let mut timed_data:Vec<TimedData> = input.lines()
      .map(|l| TimedData::new(l))
      .collect();

  timed_data.sort();

  let mut idx = 0;
  let mut guard_data: HashMap<usize, GuardHistory> = HashMap::new();
  let mut curr_id = 0;
  let mut curr_sleep_time: Option<u32> = None;
  while idx < timed_data.len() {
    match timed_data[idx].action {
      GuardAction::Shift(id) => {
        curr_id = id;
        guard_data.entry(curr_id).or_insert(GuardHistory::new());
      },
      GuardAction::Sleep(minute) => {
        curr_sleep_time = Some(minute);
      },
      GuardAction::Wakeup(minute) => {
        let st_arr = &mut guard_data.get_mut(&curr_id).unwrap().sleep_time;
        for i in curr_sleep_time.unwrap()..minute {
          st_arr[i as usize] += 1;
        }
        guard_data.get_mut(&curr_id).unwrap().total_sleep_time += minute - curr_sleep_time.unwrap();
      },
      _ => println!("What did he doooo! IDK!!!"),
    }
    idx+=1;
  }

  // find the one that sleep more than any others
  let sleepy_guard = guard_data.iter()
      .max_by(|&(_k, v), &(_k2, v2)| v.total_sleep_time.cmp(&v2.total_sleep_time));

  // which minute that he usually fall asleep
  let mmst = sleepy_guard
      .map(|(k, v)| (k, v.sleep_time))
      .map(|(k, sl)| {
        let mut max = 0;
        let mut item = (0, 0);
        for i in 0..60 {
          if sl[i] >= max {
            max = sl[i];
            item = (i, sl[i]);
          }
        }
        (k, item)
      })
      .map(|(k, v)| (k, v.0))
      .unwrap();

  println!("p1: (id, sleepy minute) {:?} - result: {:?}", mmst, mmst.0*(mmst.1 as usize));

  Ok(())
}
