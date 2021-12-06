use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Board(Vec<Vec<u32>>);

#[derive(Debug, Clone)]
struct MarkedBoard(Vec<Vec<(u32, bool)>>);

impl MarkedBoard {
    fn mark(self, called: u32) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|(number, mark)| {
                            if number == called {
                                (number, true)
                            } else {
                                (number, mark)
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn is_winner(&self) -> bool {
        let rows = self.0.iter().any(|line| line.iter().all(|(_, mark)| *mark));
        let columns = (0..self.0.len()).any(|i| self.0.iter().all(|line| line[i].1));

        rows || columns
    }

    fn score(&self, called: u32) -> u32 {
        let unmarked: u32 = self
            .0
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|(_, mark)| !mark)
                    .map(|(number, _)| number)
                    .sum::<u32>()
            })
            .sum();

        unmarked * called
    }
}

impl From<Board> for MarkedBoard {
    fn from(board: Board) -> Self {
        MarkedBoard(
            board
                .0
                .into_iter()
                .map(|line| line.into_iter().map(|num| (num, false)).collect())
                .collect(),
        )
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut blocks = input.split("\n\n");

    let numbers = blocks
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let boards = blocks
        .map(|block| {
            Board(
                block
                    .lines()
                    .map(|line| {
                        line.split_whitespace()
                            .map(str::parse)
                            .collect::<Result<_, _>>()
                            .unwrap()
                    })
                    .collect(),
            )
        })
        .collect();

    (numbers, boards)
}

#[aoc(day4, part1)]
fn part1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, boards) = input;
    let mut marked_boards: Vec<MarkedBoard> = boards
        .into_iter()
        .map(ToOwned::to_owned)
        .map(Into::into)
        .collect();

    for called in numbers {
        marked_boards = marked_boards
            .into_iter()
            .map(|board| board.mark(*called))
            .collect();

        if let Some(board) = marked_boards.iter().find(|board| board.is_winner()) {
            return board.score(*called);
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
fn part2(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, boards) = input;
    let mut marked_boards: Vec<MarkedBoard> = boards
        .into_iter()
        .map(ToOwned::to_owned)
        .map(Into::into)
        .collect();

    for called in numbers {
        let boards_before_call = marked_boards.clone();

        marked_boards = marked_boards
            .into_iter()
            .map(|board| board.mark(*called))
            .collect();

        if marked_boards.iter().all(|board| board.is_winner()) {
            for i in 0..boards_before_call.len() {
                if !boards_before_call[i].is_winner() {
                    return marked_boards[i].score(*called);
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE)), 4512);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(SAMPLE)), 1924);
    }
}
