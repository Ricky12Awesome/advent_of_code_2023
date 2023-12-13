use crate::Day;
use itertools::Itertools;

pub struct Day9 {
  nums: Vec<Vec<i64>>,
}

const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

impl<'a> Day<'a> for Day9 {
  fn setup(input: &'a str) -> Self {
    let nums = input
      .lines()
      .map(|line| {
        line
          .split_ascii_whitespace()
          .map(|n| n.parse::<i64>().unwrap())
          .collect_vec()
      })
      .collect_vec();

    Self { nums }
  }

  fn part1(&mut self) -> String {
    let mut predictions = vec![];

    for line in self.nums.clone().into_iter() {
      let mut current = line;
      let mut nums = vec![*current.last().unwrap()];

      loop {
        current = current
          .into_iter()
          .map_windows(|&[a, b]| b - a)
          .collect_vec();

        if current.iter().all(|n| *n == 0) || current.is_empty() {
          break;
        }

        nums.push(*current.last().unwrap());
      }

      // println!("{:?} {}", nums, nums.iter().sum::<i64>());
      predictions.push(nums.iter().sum::<i64>())
    }

    predictions.iter().sum::<i64>().to_string()
  }

  fn part2(&mut self) -> String {
    let mut predictions = vec![];

    for line in self.nums.clone().into_iter() {
      let mut current = line;
      let mut nums = vec![*current.first().unwrap()];

      loop {
        current = current
          .into_iter()
          .map_windows(|&[a, b]| a - b)
          .collect_vec();

        if current.iter().all(|n| *n == 0) || current.is_empty() {
          break;
        }

        nums.push(*current.first().unwrap());
      }

      // println!("{:?} {}", nums, nums.iter().sum::<i64>());
      predictions.push(nums.iter().sum::<i64>())
    }

    predictions.iter().sum::<i64>().to_string()
  }
}
