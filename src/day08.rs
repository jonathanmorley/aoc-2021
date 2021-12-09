use std::hash::{Hash, Hasher};
use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use bimap::BiHashMap;
use strum::{AsRefStr, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, AsRefStr, Eq, Hash, PartialOrd, Ord)]
#[strum(serialize_all = "lowercase")]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SignalDigit(BTreeSet<Signal>);

impl Hash for SignalDigit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for entry in self.0.iter() {
            entry.hash(state);
        }
    }
}

impl FromStr for SignalDigit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<_, _>>()
            .map(SignalDigit)
            .map_err(Into::into)
    }
}

impl SignalDigit {
    fn as_naive_u8(&self) -> Result<u8> {
        match self.as_u8s()?[..] {
            [x] => Ok(x),
            ref x => Err(anyhow!("Could be one of {:?}", x)),
        }
    }

    fn as_u8s(&self) -> Result<Vec<u8>> {
        match self.0.len() {
            2 => Ok(vec![1]),
            3 => Ok(vec![7]),
            4 => Ok(vec![4]),
            5 => Ok(vec![2, 3, 5]),
            6 => Ok(vec![0, 6, 9]),
            7 => Ok(vec![8]),
            a => Err(anyhow!("Not a valid digit with size: {}", a)),
        }
    }
}

#[derive(Clone, Debug)]
struct DisplayReading {
    patterns: HashSet<SignalDigit>,
    outputs: Vec<SignalDigit>,
    digit_map: BiHashMap<SignalDigit, u8>,
}

impl DisplayReading {
    fn add_three(&mut self) {
        let one = self.digit_map.get_by_right(&1).unwrap();
        let three = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&3))
            .find(|x| x.0.is_superset(&one.0))
            .unwrap();
        self.digit_map.insert(three.to_owned(), 3);
    }

    fn add_nine(&mut self) {
        let four = self.digit_map.get_by_right(&4).unwrap();
        let nine = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&9))
            .find(|x| x.0.is_superset(&four.0))
            .unwrap();
        self.digit_map.insert(nine.to_owned(), 9);
    }

    fn add_zero(&mut self) {
        let seven = self.digit_map.get_by_right(&7).unwrap();
        let nine = self.digit_map.get_by_right(&9).unwrap();

        let zero = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&0))
            .filter(|x| *x != nine)
            .find(|x| x.0.is_superset(&seven.0))
            .unwrap();
        self.digit_map.insert(zero.to_owned(), 0);
    }

    fn add_six(&mut self) {
        let zero = self.digit_map.get_by_right(&0).unwrap();
        let nine = self.digit_map.get_by_right(&9).unwrap();

        let six = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&6))
            .filter(|x| *x != zero)
            .filter(|x| *x != nine)
            .next()
            .unwrap();

        self.digit_map.insert(six.to_owned(), 6);
    }

    fn add_five(&mut self) {
        let six = self.digit_map.get_by_right(&6).unwrap();

        let five = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&5))
            .filter(|x| x.0.is_subset(&six.0))
            .next()
            .unwrap();

        self.digit_map.insert(five.to_owned(), 5);
    }

    fn add_two(&mut self) {
        let five = self.digit_map.get_by_right(&5).unwrap();
        let three = self.digit_map.get_by_right(&3).unwrap();

        let two = self
            .patterns
            .iter()
            .filter(|x| x.as_u8s().unwrap().contains(&5))
            .filter(|x| *x != five)
            .filter(|x| *x != three)
            .next()
            .unwrap();

        self.digit_map.insert(two.to_owned(), 2);
    }

    fn update_digit_map(&mut self) {
        for digit in &self.patterns {
            if let Ok(number) = digit.as_naive_u8() {
                self.digit_map.insert(digit.to_owned(), number);
            }
        }

        self.add_three();
        self.add_nine();
        self.add_zero();
        self.add_six();
        self.add_five();
        self.add_two();
    }

    fn reading(&self) -> u32 {
        self.outputs
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                (*self.digit_map.get_by_left(digit).unwrap() as u32) * 10u32.pow(i as u32)
            })
            .sum()
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<DisplayReading> {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(patterns, outputs)| DisplayReading {
            patterns: patterns
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            outputs: outputs
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            digit_map: BiHashMap::new(),
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[DisplayReading]) -> usize {
    input
        .iter()
        .map(|line| {
            line.outputs
                .iter()
                .filter_map(|output| output.as_naive_u8().ok())
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[DisplayReading]) -> u32 {
    let mut readings = input.to_owned();

    for reading in readings.iter_mut() {
        reading.update_digit_map();
    }

    readings.into_iter().map(|reading| reading.reading()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const SAMPLE_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn sample2_part1() {
        assert_eq!(part1(&generator(SAMPLE_2)), 26);
    }

    #[test]
    fn sample1_part2() {
        assert_eq!(part2(&generator(SAMPLE_1)), 5353);
    }

    #[test]
    fn sample2_part2() {
        assert_eq!(part2(&generator(SAMPLE_2)), 61229);
    }
}
