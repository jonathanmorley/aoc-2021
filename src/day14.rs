use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar},
    sequence::{pair, separated_pair},
    IResult,
};

type Template = Vec<char>;
type Pattern = (char, char);
type Rules = HashMap<Pattern, char>;
type PairCounts = HashMap<Pattern, usize>;

fn parse_rule(input: &str) -> IResult<&str, (Pattern, char)> {
    let (input, (pattern, result)) = separated_pair(
        pair(anychar, anychar),
        tag(" -> "),
        anychar,
    )(input)?;

    Ok((input, (pattern, result)))
}

#[aoc_generator(day14, part1)]
fn generator_part1(input: &str) -> (Template, Rules) {
    let mut blocks = input.split("\n\n");

    (
        blocks
            .next()
            .unwrap()
            .chars()
            .collect(),
        blocks
            .next()
            .unwrap()
            .lines()
            .map(|line| parse_rule(line).unwrap().1)
            .collect()
    )
}

#[aoc(day14, part1)]
fn part1((template, rules): &(Template, Rules)) -> u64 {
    let mut template = template.to_owned();
    
    for _ in 0..10 {
        for (i, window) in template.clone().windows(2).enumerate() {
            template.insert(2*i+1, rules[&(window[0], window[1])]);
        }
    }

    let counts = template.iter().counts();
    let (min, max) = counts
        .iter()
        .minmax_by_key(|(_, count)| *count)
        .into_option()
        .unwrap();
    
    (*max.1 as u64) - (*min.1 as u64)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut blocks = input.split("\n\n");

    let template = blocks
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let mut counts = template
        .windows(2)
        .map(|window| (window[0], window[1]))
        .counts();
    
    let rules: HashMap<(char, char), char> = blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| parse_rule(line).unwrap().1)
        .collect();

    for _ in 0..40 {
        for (pattern, count) in counts.clone() {
            if count > 0 {
                let left_count = counts
                    .get(&(pattern.0, rules[&pattern]))
                    .map(ToOwned::to_owned)
                    .unwrap_or_default();
                counts.insert((pattern.0, rules[&pattern]), left_count + count);

                let right_count = counts
                    .get(&(rules[&pattern], pattern.1))
                    .map(ToOwned::to_owned)
                    .unwrap_or_default();
                counts.insert((rules[&pattern], pattern.1), right_count + count);

                let pattern_count = counts
                    .get(&pattern)
                    .map(ToOwned::to_owned)
                    .unwrap();
                counts.insert(pattern, pattern_count - count);
            }
        }
    }

    let mut letter_counts = counts
        .into_iter()
        .flat_map(|((left, right), count)| [(left, count), (right, count)])
        .into_group_map()
        .into_iter()
        .map(|(c, counts)| (c, counts.into_iter().sum::<usize>() / 2))
        .collect::<HashMap<char, usize>>();

    let (first, last) = (template[0], template.last().unwrap());

    *letter_counts.get_mut(&first).unwrap() += 1;
    *letter_counts.get_mut(&last).unwrap() += 1;

    let (min, max) = letter_counts
    .iter()
    .minmax_by_key(|(_, count)| *count)
    .into_option()
    .unwrap();

    (*max.1 as u64) - (*min.1 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator_part1(SAMPLE_1)), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 2188189693529);
    }
}
