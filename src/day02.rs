use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Direction {
    Forward,
    Down,
    Up
}

#[derive(Debug, PartialEq)]
struct Vector(Direction, u32);

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        
        if let (Some(direction), Some(magnitude)) = (split.next(), split.next()) {
            Ok(Vector(direction.parse()?, magnitude.parse()?))
        } else {
            Err(anyhow!("Unable to parse '{}' as a vector", s))
        }
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Result<Vec<Vector>> {
    input.lines().map(str::parse).collect()
}

struct Location {
    horizontal: i64,
    depth: i64
}

#[aoc(day2, part1)]
fn part1(input: &[Vector]) -> i64 {
    let mut location = Location {
        horizontal: 0,
        depth: 0
    };

    for Vector(direction, magnitude) in input {
        match direction {
            Direction::Forward => location.horizontal += *magnitude as i64,
            Direction::Up => location.depth -= *magnitude as i64,
            Direction::Down => location.depth += *magnitude as i64
        }
    }

    location.horizontal * location.depth
}

struct OrientedLocation {
    horizontal: i64,
    depth: i64,
    aim: i64
}

#[aoc(day2, part2)]
fn part2(input: &[Vector]) -> i64 {
    let mut location = OrientedLocation {
        horizontal: 0,
        depth: 0,
        aim: 0
    };

    for Vector(direction, magnitude) in input {
        match direction {
            Direction::Down => location.aim += *magnitude as i64,
            Direction::Up => location.aim -= *magnitude as i64,
            Direction::Forward => {
                location.horizontal += *magnitude as i64;
                location.depth += location.aim * (*magnitude as i64)
            }
        }
    }

    location.horizontal * location.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = 
"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 150);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(SAMPLE).unwrap()), 900);
    }
}
