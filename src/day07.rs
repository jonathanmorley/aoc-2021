use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn generator(input: &str) -> Result<Vec<u32>> {
    input
        .split(",")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

#[aoc(day7, part1)]
fn part1(input: &[u32]) -> u32 {
    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());

    (min..=max)
        .map(|align| {
            input
                .iter()
                .map(|crab| (*crab as i64 - align as i64).abs() as u32)
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[u32]) -> u32 {
    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());

    (min..=max)
        .map(|align| {
            input
                .iter()
                .map(|crab| (*crab as i64 - align as i64).abs() as u32)
                .map(|distance| (distance * (distance + 1)) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 37);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(SAMPLE).unwrap()), 168);
    }
}
