use crate::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

pub struct Day3 {
  width: usize,
  height: usize,
  grid: Vec<Vec<Point>>,
  number_groups: Vec<u32>,
  number_group_lookup: HashMap<(usize, usize), usize>,
}

const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Point {
  N(char),
  Symbol(char),
  Blank,
}

impl Point {
  fn to_char(self) -> char {
    match self {
      Point::N(c) => c,
      Point::Symbol(c) => c,
      Point::Blank => '.',
    }
  }
}

impl From<char> for Point {
  fn from(value: char) -> Self {
    match value {
      '.' => Point::Blank,
      n if n.is_ascii_digit() => Point::N(value),
      s => Point::Symbol(s),
    }
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_char(self.to_char())
  }
}

impl Day3 {
  fn print_grid(&self) {
    for y in self.grid.iter() {
      for x in y.iter() {
        print!("{x}")
      }
      println!()
    }
  }

  fn find_numbers_around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    let mut numbers = Vec::new();

    let indices = [
      (x + 1, y),
      (x - 1, y),
      (x, y + 1),
      (x, y - 1),
      (x + 1, y + 1),
      (x - 1, y - 1),
      (x + 1, y - 1),
      (x - 1, y + 1),
    ];

    for (x, y) in indices {
      let Some(x): Option<usize> = x.try_into().ok() else {
        continue;
      };

      let Some(y): Option<usize> = y.try_into().ok() else {
        continue;
      };

      let Some(line) = self.grid.get(y) else {
        continue;
      };

      let Some(Point::N(_)) = line.get(x) else {
        continue;
      };

      numbers.push((x, y))
    }

    numbers
  }
}

impl<'a> Day<'a> for Day3 {
  fn setup(input: &'a str) -> Self {
    let grid = input
      .lines()
      .map(|line| line.chars().map(Point::from).collect_vec())
      .collect_vec();

    let width = grid.first().unwrap().len();
    let height = grid.len();

    let mut number_groups = Vec::new();
    let mut number_group_lookup = HashMap::new();

    for (y, line) in grid.iter().enumerate() {
      let mut iter = line.iter().enumerate();

      loop {
        iter
          .take_while_ref(|(_, point)| !matches!(point, Point::N(_)))
          .for_each(|_| {});

        let indices = iter
          .take_while_ref(|(_, point)| matches!(point, Point::N(_)))
          .map(|(x, point)| (x, point.to_char()))
          .collect_vec();

        if indices.is_empty() {
          break;
        }

        let mut str = String::new();

        for (x, c) in indices {
          str.push(c);

          number_group_lookup.insert((x, y), number_groups.len());
        }

        let n = str.parse::<u32>().unwrap();
        number_groups.push(n);
      }
    }

    Self {
      width,
      height,
      grid,
      number_groups,
      number_group_lookup,
    }
  }

  fn part1(&mut self) -> String {
    let mut indexes = HashSet::<usize>::new();

    for (y, line) in self.grid.iter().enumerate() {
      for (x, point) in line.iter().enumerate() {
        let Point::Symbol(_) = point else { continue };

        let nums = &self.find_numbers_around(x, y);

        for xy in nums {
          let i = self.number_group_lookup.get(xy).unwrap();

          indexes.insert(*i);
        }
      }
    }

    indexes
      .iter()
      .map(|&i| self.number_groups[i])
      .sum::<u32>()
      .to_string()
  }

  fn part2(&mut self) -> String {
    let mut ratios = Vec::new();

    for (y, line) in self.grid.iter().enumerate() {
      for (x, point) in line.iter().enumerate() {
        let Point::Symbol('*') = point else { continue };

        let nums = &self.find_numbers_around(x, y);

        let mut indexes = HashSet::<usize>::new();

        for xy in nums {
          let i = self.number_group_lookup.get(xy).unwrap();

          indexes.insert(*i);
        }

        let mut indexes = indexes.into_iter();

        let Some(a) = indexes.next() else {
          continue;
        };

        let Some(b) = indexes.next() else {
          continue;
        };

        let a = self.number_groups[a];
        let b = self.number_groups[b];

        ratios.push(a * b);

        println!("{indexes:?}");
      }
    }

    ratios.iter().sum::<u32>().to_string()
  }
}
