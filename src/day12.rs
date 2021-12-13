use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::aoc;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Large(&'a str),
    Small(&'a str),
}

impl Cave<'_> {
    fn is_small(&self) -> bool {
        match self {
            Cave::Start => true,
            Cave::End => true,
            Cave::Small(_) => true,
            Cave::Large(_) => false,
        }
    }

    fn is_revisitable(&self) -> bool {
        match self {
            Cave::Start => false,
            Cave::End => false,
            Cave::Small(_) => true,
            Cave::Large(_) => true,
        }
    }
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            n if n == n.to_lowercase() => Cave::Small(n),
            n if n == n.to_uppercase() => Cave::Large(n),
            _ => unreachable!(),
        }
    }
}

fn paths<'a>(
    graph: &UnGraphMap<Cave<'a>, ()>,
    initial: Vec<Cave<'a>>,
    to: Cave<'a>,
) -> HashSet<Vec<Cave<'a>>> {
    graph
        .neighbors(initial.last().unwrap().to_owned())
        .filter(|neighbour| !(neighbour.is_small() && initial.contains(neighbour)))
        .flat_map(|neighbour| {
            let mut path = initial.clone();
            path.push(neighbour);

            if neighbour == to {
                [path].into()
            } else {
                paths(graph, path, to)
            }
        })
        .collect()
}

fn paths_with_single_revisit<'a>(
    graph: &UnGraphMap<Cave<'a>, ()>,
    initial: Vec<Cave<'a>>,
    to: Cave<'a>,
) -> HashSet<Vec<Cave<'a>>> {
    graph
        .neighbors(initial.last().unwrap().to_owned())
        .flat_map(|neighbour| {
            let mut path = initial.clone();
            path.push(neighbour);

            if neighbour == to {
                [path].into()
            } else if initial.contains(&neighbour) && !neighbour.is_revisitable() {
                // revisit of start or end
                [].into()
            } else if initial.contains(&neighbour) && neighbour.is_small() {
                // revisit of a small cave
                paths(graph, path, to)
            } else {
                paths_with_single_revisit(graph, path, to)
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1<'a>(input: &'a str) -> usize {
    let graph: UnGraphMap<Cave<'a>, ()> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(from, to)| (from.into(), to.into()))
        .collect();

    paths(&graph, vec![Cave::Start], Cave::End).len()
}

#[aoc(day12, part2)]
fn part2<'a>(input: &'a str) -> usize {
    let graph: UnGraphMap<Cave<'a>, ()> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(from, to)| (from.into(), to.into()))
        .collect();

    paths_with_single_revisit(&graph, vec![Cave::Start], Cave::End).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const SAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const SAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 10);
        assert_eq!(part1(SAMPLE_2), 19);
        assert_eq!(part1(SAMPLE_3), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_1), 36);
        assert_eq!(part2(SAMPLE_2), 103);
        assert_eq!(part2(SAMPLE_3), 3509);
    }
}
