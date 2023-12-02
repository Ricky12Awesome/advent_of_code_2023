mod day1;

use crate::day1::Day1;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Which day you want to run
  #[arg(short, long)]
  day: u32,

  /// Which part you want to run, 0 to run all parts
  #[arg(short, long, default_value_t = 1)]
  part: u32,

  /// Override input with custom input
  input: Option<String>,
}

pub trait Day {
  fn setup(input: &str) -> Self;

  fn part1(&mut self) -> String;
  fn part2(&mut self) -> String;
}

pub const DEFAULT_INPUTS: &[&str] = &[
  "", // Day 0, just so I don't have to do -1 when indexing
  include_str!("../inputs/1.txt"),
];

pub fn run_day<D: Day>(input: &str, day_n: u32, part: u32) -> anyhow::Result<()> {
  let mut day = D::setup(input);

  match part {
    0 => {
      println!("Day {} Part 1: {}", day_n, day.part1());
      println!("Day {} Part 2: {}", day_n, day.part2());
    }
    1 => {
      println!("Day {} Part 1: {}", day_n, day.part1());
    }
    2 => {
      println!("Day {} Part 2: {}", day_n, day.part2());
    }
    _ => return Err(anyhow::Error::msg("Invalid Part")),
  }

  Ok(())
}

pub fn main() -> anyhow::Result<()> {
  let args = Args::try_parse()?;
  let input = args
    .input
    .as_deref()
    .unwrap_or_else(|| DEFAULT_INPUTS[args.day as usize]);

  match args.day {
    1 => run_day::<Day1>(input, args.day, args.part),
    _ => Err(anyhow::Error::msg("Invalid Day")),
  }
}
