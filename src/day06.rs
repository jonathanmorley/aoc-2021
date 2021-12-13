use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{ArrayStorage, SMatrix, SVector};

#[aoc_generator(day6)]
fn generator(input: &str) -> [u64; 9] {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .fold([0; 9], |mut acc, fish: usize| {
            acc[fish] += 1;
            acc
        })
}

fn lanternfish(initial: Vec<u64>, day: u32) -> u64 {
    let mut school = initial;

    for _ in 0..day {
        school.rotate_left(1);
        school[6] += school[8];
    }

    school.into_iter().sum()
}

#[aoc(day6, part1)]
fn part1(input: &[u64; 9]) -> u64 {
    lanternfish(Vec::from_iter(input.to_owned()), 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u64; 9]) -> u64 {
    lanternfish(Vec::from_iter(input.to_owned()), 256)
}

const LANTERNFISH_MATRIX: SMatrix<u64, 9, 9> = SMatrix::from_array_storage(ArrayStorage([
    [0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0],
]));

#[aoc(day6, part2, matrix)]
fn part2_matrix(input: &[u64; 9]) -> u64 {
    let mut m = LANTERNFISH_MATRIX;
    (1..256).for_each(|_| m *= LANTERNFISH_MATRIX);

    (m * SVector::from_column_slice(input)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE)), 5934);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(SAMPLE)), 26984457539);
    }

    #[test]
    fn matrix2() {
        assert_eq!(part2_matrix(&generator(SAMPLE)), 26984457539);
    }
}
