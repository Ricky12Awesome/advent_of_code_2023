#![feature(exact_size_is_empty)]

mod day1;
mod day2;
mod day3;

use crate::day1::Day1;
use clap::Parser;
use crate::day2::Day2;
use crate::day3::Day3;

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
  #[arg(short, long)]
  input: Option<String>,
}

pub trait Day<'a> {
  fn setup(input: &'a str) -> Self;

  fn part1(&mut self) -> String;
  fn part2(&mut self) -> String;
}

pub const DEFAULT_INPUTS: &[&str] = &[
  "", // Day 0, just so I don't have to do -1 when indexing
  include_str!("../inputs/1.txt"),
  include_str!("../inputs/2.txt"),
  include_str!("../inputs/3.txt"),
  // include_str!("../inputs/4.txt"),
  // include_str!("../inputs/5.txt"),
  // include_str!("../inputs/6.txt"),
  // include_str!("../inputs/7.txt"),
  // include_str!("../inputs/8.txt"),
  // include_str!("../inputs/9.txt"),
  // include_str!("../inputs/10.txt"),
  // include_str!("../inputs/11.txt"),
  // include_str!("../inputs/12.txt"),
  // include_str!("../inputs/13.txt"),
  // include_str!("../inputs/14.txt"),
  // include_str!("../inputs/15.txt"),
  // include_str!("../inputs/16.txt"),
  // include_str!("../inputs/17.txt"),
  // include_str!("../inputs/18.txt"),
  // include_str!("../inputs/19.txt"),
  // include_str!("../inputs/20.txt"),
  // include_str!("../inputs/21.txt"),
  // include_str!("../inputs/22.txt"),
  // include_str!("../inputs/23.txt"),
  // include_str!("../inputs/24.txt"),
  // include_str!("../inputs/25.txt"),
];

pub fn run_day<'a, D: Day<'a>>(input: &'a str, day_n: u32, part: u32) -> anyhow::Result<()> {
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
    2 => run_day::<Day2>(input, args.day, args.part),
    3 => run_day::<Day3>(input, args.day, args.part),
    // 4 => run_day::<Day4 >(input, args.day, args.part),
    // 5 => run_day::<Day5 >(input, args.day, args.part),
    // 6 => run_day::<Day6 >(input, args.day, args.part),
    // 7 => run_day::<Day7 >(input, args.day, args.part),
    // 8 => run_day::<Day8 >(input, args.day, args.part),
    // 9 => run_day::<Day9 >(input, args.day, args.part),
    // 10 => run_day::<Day10>(input, args.day, args.part),
    // 11 => run_day::<Day11>(input, args.day, args.part),
    // 12 => run_day::<Day12>(input, args.day, args.part),
    // 13 => run_day::<Day13>(input, args.day, args.part),
    // 14 => run_day::<Day14>(input, args.day, args.part),
    // 15 => run_day::<Day15>(input, args.day, args.part),
    // 16 => run_day::<Day16>(input, args.day, args.part),
    // 17 => run_day::<Day17>(input, args.day, args.part),
    // 18 => run_day::<Day18>(input, args.day, args.part),
    // 19 => run_day::<Day19>(input, args.day, args.part),
    // 20 => run_day::<Day20>(input, args.day, args.part),
    // 21 => run_day::<Day21>(input, args.day, args.part),
    // 22 => run_day::<Day22>(input, args.day, args.part),
    // 23 => run_day::<Day23>(input, args.day, args.part),
    // 24 => run_day::<Day24>(input, args.day, args.part),
    // 25 => run_day::<Day25>(input, args.day, args.part),
    _ => Err(anyhow::Error::msg("Invalid Day")),
  }
}
