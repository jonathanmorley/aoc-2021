use std::cmp::Ordering;
use std::{collections::HashSet, fmt, ops::RangeInclusive};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Dot(i32, i32);

fn parse_dot(input: &str) -> IResult<&str, Dot> {
    let (input, (x, y)) = separated_pair(
        map_res(digit1, str::parse),
        char(','),
        map_res(digit1, str::parse),
    )(input)?;
    Ok((input, Dot(x, y)))
}

impl Dot {
    fn fold(&self, fold: &Fold) -> Option<Dot> {
        match fold {
            Fold::Left(mid) => match self.0.cmp(mid) {
                Ordering::Less => Some(Dot(self.0, self.1)),
                Ordering::Equal => None,
                Ordering::Greater => Some(Dot(2 * mid - self.0, self.1)),
            },
            Fold::Up(mid) => match self.1.cmp(mid) {
                Ordering::Less => Some(Dot(self.0, self.1)),
                Ordering::Equal => None,
                Ordering::Greater => Some(Dot(self.0, 2 * mid - self.1)),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Paper(HashSet<Dot>);

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in self.height() {
            for x in self.width() {
                if self.0.contains(&Dot(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Paper {
    fn width(&self) -> RangeInclusive<i32> {
        let (min, max) = (
            self.0.iter().map(|Dot(x, _)| *x).min().unwrap(),
            self.0.iter().map(|Dot(x, _)| *x).max().unwrap(),
        );

        min..=max
    }

    fn height(&self) -> RangeInclusive<i32> {
        let (min, max) = (
            self.0.iter().map(|Dot(_, y)| *y).min().unwrap(),
            self.0.iter().map(|Dot(_, y)| *y).max().unwrap(),
        );

        min..=max
    }

    fn fold(&mut self, fold: &Fold) {
        *self = Paper(self.0.iter().filter_map(|dot| dot.fold(fold)).collect())
    }
}

#[derive(Debug)]
enum Fold {
    Up(i32),
    Left(i32),
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    let (input, (dimension, position)) = preceded(
        tag("fold along "),
        separated_pair(anychar, char('='), map_res(digit1, str::parse)),
    )(input)?;

    let fold = match dimension {
        'x' => Fold::Left(position),
        'y' => Fold::Up(position),
        _ => unreachable!(),
    };

    Ok((input, fold))
}

#[aoc_generator(day13)]
fn generator(input: &str) -> (Paper, Vec<Fold>) {
    let mut blocks = input.split("\n\n");

    (
        Paper(
            blocks
                .next()
                .unwrap()
                .lines()
                .map(|line| parse_dot(line).unwrap().1)
                .collect(),
        ),
        blocks
            .next()
            .unwrap()
            .lines()
            .map(|line| parse_fold(line).unwrap().1)
            .collect(),
    )
}

#[aoc(day13, part1)]
fn part1((paper, folds): &(Paper, Vec<Fold>)) -> usize {
    let mut paper = paper.to_owned();

    paper.fold(&folds[0]);

    paper.0.len()
}

#[aoc(day13, part2)]
fn part2((paper, folds): &(Paper, Vec<Fold>)) -> String {
    let mut paper = paper.to_owned();

    for fold in folds {
        paper.fold(fold);
    }

    String::from("\n") + &paper.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE_1)), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&generator(SAMPLE_1)),
            "
#####
#...#
#...#
#...#
#####
"
        );
    }
}
