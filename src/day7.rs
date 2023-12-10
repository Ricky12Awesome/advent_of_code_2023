use indexmap::{Equivalent, IndexMap};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};

use itertools::Itertools;

use crate::Day;

pub struct Day7<'a> {
  input: &'a str,
}

const EXAMPLE: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"#;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(usize)]
enum Card {
  A = 14,
  K = 13,
  Q = 12,
  T = 10,
  J = 11,
  N9 = 9,
  N8 = 8,
  N7 = 7,
  N6 = 6,
  N5 = 5,
  N4 = 4,
  N3 = 3,
  N2 = 2,
  JWeak = 1,
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::A => f.write_char('A'),
      Self::K => f.write_char('K'),
      Self::Q => f.write_char('Q'),
      Self::T => f.write_char('T'),
      Self::J => f.write_char('J'),
      Self::JWeak => f.write_char('J'),
      Self::N9 => f.write_char('9'),
      Self::N8 => f.write_char('8'),
      Self::N7 => f.write_char('7'),
      Self::N6 => f.write_char('6'),
      Self::N5 => f.write_char('5'),
      Self::N4 => f.write_char('4'),
      Self::N3 => f.write_char('3'),
      Self::N2 => f.write_char('2'),
    }
  }
}

impl Card {
  fn new(c: char, joker_weak: bool) -> Self {
    match c {
      'A' => Self::A,
      'K' => Self::K,
      'Q' => Self::Q,
      'T' => Self::T,
      'J' if !joker_weak => Self::J,
      'J' if joker_weak => Self::JWeak,
      '9' => Self::N9,
      '8' => Self::N8,
      '7' => Self::N7,
      '6' => Self::N6,
      '5' => Self::N5,
      '4' => Self::N4,
      '3' => Self::N3,
      '2' => Self::N2,
      _ => unreachable!(),
    }
  }

  fn _eq(&self, other: &Self) -> bool {
    let lhs = *self as usize;
    let rhs = *other as usize;
    let joker = Card::JWeak as usize;

    if lhs == joker || rhs == joker {
      true
    } else {
      lhs.eq(&rhs)
    }
  }
}

impl From<char> for Card {
  fn from(c: char) -> Self {
    Self::new(c, false)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
  cards: [Card; 5],
}

impl Display for Hand {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let [a, b, c, d, e] = &self.cards;

    Display::fmt(a, f)?;
    Display::fmt(b, f)?;
    Display::fmt(c, f)?;
    Display::fmt(d, f)?;
    Display::fmt(e, f)
  }
}

impl Hand {
  fn new(cards: &str, joker_wildcard: bool) -> Self {
    Self {
      cards: cards
        .chars()
        .map(|c| Card::new(c, joker_wildcard))
        .array_chunks::<5>()
        .next()
        .unwrap(),
    }
  }

  fn strength(&self) -> usize {
    let mut counts = self.cards.into_iter().counts();

    let n_jokers = counts.get(&Card::JWeak).copied();
    let highest = counts
      .iter()
      .filter(|(c, _)| !matches!(c, Card::JWeak))
      .max_by(|(_, a), (_, b)| a.cmp(b));

    if let (Some(n_jokers), Some((&card, _))) = (n_jokers, highest) {
      *counts.get_mut(&card).unwrap() += n_jokers;
      counts.remove(&Card::JWeak);
    };

    let mut values = counts.values().copied().collect_vec();

    values.sort_unstable();
    values.reverse();

    match values.as_slice() {
      [5] => 7,
      [4, 1] => 6,
      [3, 2] => 5,
      [3, 1, 1] => 4,
      [2, 2, 1] => 3,
      [2, 1, 1, 1] => 2,
      [1, 1, 1, 1, 1] => 1,
      _ => unreachable!(),
    }
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd<Self> for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.strength().partial_cmp(&other.strength()) {
      Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
      order => order,
    }
  }
}

impl<'a> Day<'a> for Day7<'a> {
  fn setup(input: &'a str) -> Self {
    // Self { input: EXAMPLE }
    Self { input }
  }

  // Real: 253205868
  // Example: 6592
  fn part1(&mut self) -> String {
    let mut lines = self
      .input
      .lines()
      .map(|line| line.split_once(' ').unwrap())
      .map(|(cards, bid)| (Hand::new(cards, false), bid.parse::<usize>().unwrap()))
      .collect_vec();

    lines.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    lines
      .iter()
      .enumerate()
      .map(|(index, &(_, bid))| bid * (index + 1))
      .sum::<usize>()
      .to_string()
  }

  // Real: 253907829
  // Example: 6839
  fn part2(&mut self) -> String {
    let mut lines = self
      .input
      .lines()
      .map(|line| line.split_once(' ').unwrap())
      .map(|(cards, bid)| (Hand::new(cards, true), bid.parse::<usize>().unwrap()))
      .collect_vec();

    lines.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    lines
      .iter()
      .enumerate()
      .map(|(index, &(_, bid))| bid * (index + 1))
      .sum::<usize>()
      .to_string()
  }
}