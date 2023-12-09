use crate::Day;
use itertools::Itertools;

pub struct Day6<'a> {
  time_line: &'a str,
  distance_line: &'a str,
}

const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

impl<'a> Day<'a> for Day6<'a> {
  fn setup(input: &'a str) -> Self {
    let mut lines = input.lines().collect_vec();

    Self {
      time_line: lines[0],
      distance_line: lines[1],
    }
  }

  fn part1(&mut self) -> String {
    self
      .time_line
      .split_ascii_whitespace()
      .dropping(1)
      .map(|n| n.parse::<u32>().unwrap())
      .interleave(
        self
          .distance_line
          .split_ascii_whitespace()
          .dropping(1)
          .map(|n| n.parse::<u32>().unwrap()),
      )
      .array_chunks::<2>()
      .map(|[time, distance]| (1..=time).filter(|&n| (time - n) * n > distance).count())
      .product::<usize>()
      .to_string()
  }

  fn part2(&mut self) -> String {
    let time = self.time_line[5..].replace(' ', "").parse::<usize>().unwrap();

    let distance = self.distance_line["Distance:".len()..]
      .replace(' ', "")
      .parse::<usize>()
      .unwrap();

    (1..=time)
      .filter(|&n| (time - n) * n > distance)
      .count()
      .to_string()
  }
}
