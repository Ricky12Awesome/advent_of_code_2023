#![feature(exact_size_is_empty)]
#![feature(array_chunks)]
#![feature(iter_collect_into)]
#![feature(iter_array_chunks)]
#![feature(slice_group_by)]
#![feature(iter_map_windows)]
#![allow(unused)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use crate::day1::Day1;
use crate::day10::Day10;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day9::Day9;
use clap::Parser;
use std::time::Duration;

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

  /// Benchmark
  #[arg(short, long)]
  bench: bool,

  /// Benchmark time in ms
  #[arg(short, long, default_value_t = 2000)]
  time: u64,
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
  include_str!("../inputs/4.txt"),
  include_str!("../inputs/5.txt"),
  include_str!("../inputs/6.txt"),
  include_str!("../inputs/7.txt"),
  include_str!("../inputs/8.txt"),
  include_str!("../inputs/9.txt"),
  include_str!("../inputs/10.txt"),
  include_str!("../inputs/11.txt"),
  include_str!("../inputs/12.txt"),
  include_str!("../inputs/13.txt"),
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

pub fn run_benchmark<'a, D: Day<'a>>(input: &'a str, args: &Args) -> anyhow::Result<()> {
  match args.part {
    0 => {
      let result =
        benchmarking::bench_function_with_duration(Duration::from_millis(args.time), |m| {
          m.measure(|| {
            let mut d = D::setup(input);

            d.part1();
            d.part2()
          });
        })?;

      println!(
        "[Benchmark] Day {} Part 1 & 2: {:?}",
        args.day,
        result.elapsed()
      );
    }
    1 => {
      let result =
        benchmarking::bench_function_with_duration(Duration::from_millis(args.time), |m| {
          m.measure(|| {
            D::setup(input).part1();
          });
        })?;

      println!(
        "[Benchmark] Day {} Part 1: {:?}",
        args.day,
        result.elapsed()
      );
    }
    2 => {
      let result =
        benchmarking::bench_function_with_duration(Duration::from_millis(args.time), |m| {
          m.measure(|| {
            D::setup(input).part2();
          });
        })?;

      println!(
        "[Benchmark] Day {} Part 2: {:?}",
        args.day,
        result.elapsed()
      );
    }
    _ => return Err(anyhow::Error::msg("Invalid Part")),
  }

  Ok(())
}

pub fn run_day<'a, D: Day<'a>>(input: &'a str, args: &Args) -> anyhow::Result<()> {
  if args.bench {
    return run_benchmark::<D>(input, args);
  }

  let mut day = D::setup(input);

  match args.part {
    0 => {
      println!("Day {} Part 1: {}", args.day, day.part1());
      println!("Day {} Part 2: {}", args.day, day.part2());
    }
    1 => {
      println!("Day {} Part 1: {}", args.day, day.part1());
    }
    2 => {
      println!("Day {} Part 2: {}", args.day, day.part2());
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
    1 => run_day::<Day1>(input, &args),
    2 => run_day::<Day2>(input, &args),
    3 => run_day::<Day3>(input, &args),
    4 => run_day::<Day4>(input, &args),
    5 => run_day::<Day5>(input, &args),
    6 => run_day::<Day6>(input, &args),
    7 => run_day::<Day7>(input, &args),
    8 => run_day::<Day8>(input, &args),
    9 => run_day::<Day9>(input, &args),
    10 => run_day::<Day10>(input, &args),
    11 => run_day::<Day11>(input, &args),
    12 => run_day::<Day12>(input, &args),
    13 => run_day::<Day13>(input, &args),
    // 14 => run_day::<Day14>(input, &args),
    // 15 => run_day::<Day15>(input, &args),
    // 16 => run_day::<Day16>(input, &args),
    // 17 => run_day::<Day17>(input, &args),
    // 18 => run_day::<Day18>(input, &args),
    // 19 => run_day::<Day19>(input, &args),
    // 20 => run_day::<Day20>(input, &args),
    // 21 => run_day::<Day21>(input, &args),
    // 22 => run_day::<Day22>(input, &args),
    // 23 => run_day::<Day23>(input, &args),
    // 24 => run_day::<Day24>(input, &args),
    // 25 => run_day::<Day25>(input, &args),
    _ => Err(anyhow::Error::msg("Invalid Day")),
  }
}
