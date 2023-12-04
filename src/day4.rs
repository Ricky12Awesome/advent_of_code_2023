use crate::Day;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day4 {
  cards: Vec<Card>,
}

const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

#[derive(Debug, Default)]
struct Card {
  winning: Vec<u32>,
  numbers: Vec<u32>,
}

impl Card {
  fn new(line: &str) -> Self {
    let (_, rest) = line.split_once(": ").unwrap();
    let (winning, numbers) = rest.split_once(" | ").unwrap();

    let winning = winning
      .split_ascii_whitespace()
      .map(|n| n.parse::<u32>().unwrap())
      .collect_vec();

    let numbers = numbers
      .split_ascii_whitespace()
      .map(|n| n.parse::<u32>().unwrap())
      .collect_vec();

    Self { winning, numbers }
  }

  fn count_winning(&self) -> usize {
    self
      .numbers
      .iter()
      .filter(|n| self.winning.contains(n))
      .count()
  }

  fn count_winning_score(&self) -> u32 {
    let numbers = self.count_winning();

    if numbers > 0 {
      2u32.pow(numbers as u32 - 1)
    } else {
      0
    }
  }
}

impl<'a> Day<'a> for Day4 {
  fn setup(input: &'a str) -> Self {
    let cards = input.lines().map(Card::new).collect_vec();

    Self { cards }
  }

  fn part1(&mut self) -> String {
    self
      .cards
      .iter()
      .map(Card::count_winning_score)
      .sum::<u32>()
      .to_string()
  }

  fn part2(&mut self) -> String {
    let mut cards = vec![1; self.cards.len()];

    for (i, card) in self.cards.iter().enumerate() {
      let winning = card.count_winning();
      let times = cards[i];

      for j in 1..=winning {
        if let Some(n) = cards.get_mut(i + j) {
          *n += times;
        }
      }
    }

    cards.iter().sum::<usize>().to_string()
  }
}
