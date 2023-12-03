use crate::Day;
use itertools::Itertools;

pub struct Day2<'a> {
  games: Vec<Vec<Vec<(i32, &'a str)>>>,
}

const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

fn parse_line(line: &str) -> Vec<Vec<(i32, &str)>> {
  let (_, sets) = line.split_once(':').unwrap();

  sets
    .split(';')
    .map(|set| set.split(','))
    .map(|set| set.map(str::trim))
    .map(|set| set.filter_map(|set| set.split_once(' ')))
    .map(|set| set.map(|(n, color)| (n.parse::<_>().unwrap(), color)))
    .map(|set| set.collect_vec())
    .collect_vec()
}

impl<'a> Day<'a> for Day2<'a> {
  fn setup(input: &'a str) -> Self {
    Day2 {
      games: input.lines().map(parse_line).collect_vec(),
      // games: EXAMPLE.lines().map(parse_line).collect_vec(),
    }
  }

  fn part1(&mut self) -> String {
    let mut sum = 0;

    'games: for (i, sets) in self.games.iter().enumerate() {
      let i = i + 1;

      for set in sets {
        let mut red = 12;
        let mut green = 13;
        let mut blue = 14;

        for &(n, color) in set {
          match color {
            "red" => red -= n,
            "green" => green -= n,
            "blue" => blue -= n,
            _ => unreachable!(),
          }
        }

        if [red, green, blue].iter().any(|&n| n.is_negative()) {
          continue 'games;
        }
      }

      sum += i;
    }

    sum.to_string()
  }

  fn part2(&mut self) -> String {
    let mut sum = 0;

    for sets in self.games.iter() {
      let mut red = 0;
      let mut green = 0;
      let mut blue = 0;

      for set in sets {
        for &(n, color) in set {
          match color {
            "red" if n > red => red = n,
            "green" if n > green => green = n,
            "blue" if n > blue => blue = n,
            _ => (),
          }
        }
      }

      sum += red * green * blue
    }

    sum.to_string()
  }
}
