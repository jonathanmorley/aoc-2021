use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generator(input: &str) -> Result<Vec<u32>> {
    input
        .split(",")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

fn lanternfish(initial: Vec<u32>, day: u32) -> Vec<u32> {
  let mut lanternfish = initial; 

  for _ in 0..(day / 7) {
    lanternfish = lanternfish_week(lanternfish);
    dbg!(lanternfish.len());
  }

  for _ in 0..(day % 7) {
    lanternfish = lanternfish_day(lanternfish);
  }

  lanternfish
}

fn lanternfish_day(initial: Vec<u32>) -> Vec<u32> {
  initial
    .into_iter()
    .flat_map(|lanternfish| if lanternfish == 0 {
      vec![6, 8]
    } else {
      vec![lanternfish - 1]
    })
    .collect()
}

fn lanternfish_week(initial: Vec<u32>) -> Vec<u32> {
  let new = initial
    .clone()
    .into_iter()
    .map(|fish| (fish + 2))
    .filter(|fish| fish <= &8);

  initial
    .into_iter()
    .map(|fish| fish % 7)
    .chain(new)
    .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[u32]) -> usize {
  lanternfish(input.to_owned(), 80).len()
}

#[aoc(day6, part2)]
fn part2(input: &[u32]) -> usize {
  lanternfish(input.to_owned(), 256).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 5934);
        assert!(false);
    }

    #[test]
    fn trivial() {
        assert_eq!(part1(&[8]), 768);
    }

    // #[test]
    // fn sample2() {
    //     assert_eq!(part2(&generator(SAMPLE).unwrap()), 26984457539);
    // }
}
