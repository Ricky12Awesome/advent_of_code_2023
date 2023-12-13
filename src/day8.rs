use crate::Day;
use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Day8<'a> {
  instructions: Vec<Instruction>,
  network: IndexMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
  Left,
  Right,
}

impl From<char> for Instruction {
  fn from(value: char) -> Self {
    match value {
      'L' => Self::Left,
      'R' => Self::Right,
      _ => unreachable!(),
    }
  }
}

const EXAMPLE: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

impl<'a> Day<'a> for Day8<'a> {
  fn setup(input: &'a str) -> Self {
    let mut lines = input.lines();
    let instructions = lines
      .next()
      .unwrap()
      .chars()
      .map(Instruction::from)
      .collect_vec();

    let network = lines
      .skip(1)
      .map(|line| {
        let (src, dst) = line.split_once(" = ").unwrap();
        let (left, right) = dst
          .trim_start_matches('(')
          .trim_end_matches(')')
          .split_once(", ")
          .unwrap();

        (src, (left, right))
      })
      .collect::<IndexMap<_, _>>();

    Self {
      instructions,
      network,
    }
  }

  #[allow(clippy::explicit_counter_loop)]
  fn part1(&mut self) -> String {
    let instructions = self.instructions.iter().cycle();

    let mut current = ("AAA", self.network.get("AAA").unwrap());
    let mut counter = 0;

    for instruction in instructions {
      let (src, (left, right)) = current;

      if src == "ZZZ" {
        break;
      }

      counter += 1;
      current = match instruction {
        Instruction::Left => (left, self.network.get(left).unwrap()),
        Instruction::Right => (right, self.network.get(right).unwrap()),
      };
    }

    counter.to_string()
  }

  fn part2(&mut self) -> String {
    let instructions = self.instructions.iter().cycle();

    let mut currents = self
      .network
      .iter()
      .filter(|(src, _)| src.ends_with('A'))
      .collect_vec();

    let mut counters = vec![(0, false); currents.len()];

    for instruction in instructions {
      if counters.iter().all(|(_, ended)| *ended) {
        break;
      }

      for (current, (counter, ended)) in currents.iter_mut().zip(counters.iter_mut()) {
        let (src, (left, right)) = current;

        if src.ends_with('Z') {
          *ended = true;
        }

        if *ended {
          continue;
        }

        *counter += 1;
        *current = match instruction {
          Instruction::Left => (left, &self.network[left]),
          Instruction::Right => (right, &self.network[right]),
        };
      }
    }

    iter_lcm(counters.into_iter().map(|(n, _)| n)).to_string()
  }
}

fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn lcm(a: u64, b: u64) -> u64 {
  if a == 0 || b == 0 {
    0
  } else {
    (a * b) / gcd(a, b)
  }
}

fn iter_lcm(numbers: impl Iterator<Item = u64>) -> u64 {
  numbers.fold(1, lcm)
}
