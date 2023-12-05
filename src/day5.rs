use crate::Day;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;

pub struct Day5<'a> {
  input: Vec<Vec<&'a str>>,
  seeds: Vec<usize>,
}

const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

#[derive(Debug, Clone)]
struct Map {
  dest: Range<usize>,
  src: Range<usize>,
}

fn lookup(n: usize, maps: &[Map]) -> usize {
  let Some(map) = maps.iter().find(|map| map.src.contains(&n)) else {
    return n;
  };

  let distance = n - map.src.start;

  map.dest.start + distance
}

fn parse(nums: &str) -> Map {
  let (a, rest) = nums.split_once(' ').unwrap();
  let (b, len) = rest.split_once(' ').unwrap();
  let a = a.parse().unwrap();
  let b = b.parse().unwrap();
  let len = len.parse::<usize>().unwrap();

  Map {
    dest: a..a + len,
    src: b..b + len,
  }
}

impl<'a> Day5<'a> {
  #[rustfmt::skip]
  fn min_location(&self, seeds: impl ParallelIterator<Item = usize>) -> usize {
    let seed_to_soil = self.input[1][1..].iter().copied().map(parse).collect_vec();
    let soil_to_fertilizer = self.input[2][1..].iter().copied().map(parse).collect_vec();
    let fertilizer_to_water = self.input[3][1..].iter().copied().map(parse).collect_vec();
    let water_to_light = self.input[4][1..].iter().copied().map(parse).collect_vec();
    let light_to_temperature = self.input[5][1..].iter().copied().map(parse).collect_vec();
    let temperature_to_humidity = self.input[6][1..].iter().copied().map(parse).collect_vec();
    let humidity_to_location = self.input[7][1..].iter().copied().map(parse).collect_vec();

    let humidity_to_location = seeds
      .map(move |n| lookup(n, &seed_to_soil))
      .map(move |n| lookup(n, &soil_to_fertilizer))
      .map(move |n| lookup(n, &fertilizer_to_water))
      .map(move |n| lookup(n, &water_to_light))
      .map(move |n| lookup(n, &light_to_temperature))
      .map(move |n| lookup(n, &temperature_to_humidity))
      .map(move |n| lookup(n, &humidity_to_location));

    humidity_to_location.min().unwrap()
  }
}
impl<'a> Day<'a> for Day5<'a> {
  fn setup(input: &'a str) -> Self {
    let input = input.lines().collect_vec();
    let input = input
      .split(|line| line.is_empty())
      .map(Vec::from)
      .collect_vec();

    let seeds = input[0][0]["seeds: ".len()..]
      .split_ascii_whitespace()
      .map(|n| n.parse::<usize>().unwrap())
      .collect_vec();

    Self { input, seeds }
  }

  fn part1(&mut self) -> String {
    // 265018614
    self
      .min_location(self.seeds.par_iter().copied())
      .to_string()
  }

  fn part2(&mut self) -> String {
    let seeds = self
      .seeds
      .array_chunks::<2>()
      .flat_map(|&[a, b]| a..a + b)
      .par_bridge();

    self.min_location(seeds).to_string()
  }
}
