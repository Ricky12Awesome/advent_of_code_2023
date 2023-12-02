use crate::Day;
use itertools::Itertools;
use std::fmt::Write;

pub struct Day1(String);

const EXAMPLE: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const EXAMPLE2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

const NAMED: [&str; 10] = [
  "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn named_to_n(line: &str) -> String {
  let mut buf = line.to_string();

  buf.write_str("     ").unwrap();

  loop {
    let Some((index, name)) = buf
      .chars()
      .collect_vec()
      .windows(5)
      .map(|it| it.iter().collect::<String>())
      .map(|it| NAMED.iter().find(|&name| it.starts_with(name)))
      .find_position(|it| it.is_some())
    else {
      break;
    };

    let name = name.unwrap();
    let n = NAMED.iter().position(|n| n == name).unwrap();

    buf.replace_range(index..index + name.len() - 1, &n.to_string());
    println!("{buf}")
  }

  buf
}

impl Day for Day1 {
  fn setup(input: &str) -> Self {
    Day1(input.into())
  }

  fn part1(&mut self) -> String {
    self
      .0
      .lines()
      .map(|line| {
        line
          .chars()
          .filter(|c| c.is_ascii_digit())
          .collect::<String>()
      })
      .map(|line| format!("{}{}", &line[0..1], &line[line.len() - 1..line.len()]))
      .map(|line| line.parse::<u32>().unwrap())
      .sum::<u32>()
      .to_string()
  }

  fn part2(&mut self) -> String {
    self
      .0
      .lines()
      .map(named_to_n)
      .map(|line| {
        line
          .chars()
          .filter(|c| c.is_ascii_digit())
          .collect::<String>()
      })
      .map(|line| format!("{}{}", &line[0..1], &line[line.len() - 1..line.len()]))
      .map(|line| line.parse::<u32>().unwrap())
      .sum::<u32>()
      .to_string()
  }
}
