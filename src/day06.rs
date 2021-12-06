use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

struct LanternSchool([usize; 9]);

impl From<Vec<u32>> for LanternSchool {
  fn from(vec: Vec<u32>) -> Self {
    LanternSchool([
      vec.iter().filter(|x| *x == &0).count(),
      vec.iter().filter(|x| *x == &1).count(),
      vec.iter().filter(|x| *x == &2).count(),
      vec.iter().filter(|x| *x == &3).count(),
      vec.iter().filter(|x| *x == &4).count(),
      vec.iter().filter(|x| *x == &5).count(),
      vec.iter().filter(|x| *x == &6).count(),
      vec.iter().filter(|x| *x == &7).count(),
      vec.iter().filter(|x| *x == &8).count(),
    ])
  }
}

impl LanternSchool {
  fn day(self) -> Self {
    LanternSchool([
      self.0[1],
      self.0[2],
      self.0[3],
      self.0[4],
      self.0[5],
      self.0[6],
      self.0[7] + self.0[0],
      self.0[8],
      self.0[0]
    ])
  }
}

#[aoc_generator(day6)]
fn generator(input: &str) -> Result<Vec<u32>> {
    input
        .split(",")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

fn lanternfish(initial: Vec<u32>, day: u32) -> usize {
  let mut school: LanternSchool = initial.into();

  for _ in 0..day {
    school = school.day();
  }

  school.0.into_iter().sum()
}


#[aoc(day6, part1)]
fn part1(input: &[u32]) -> usize {
  lanternfish(input.to_owned(), 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u32]) -> usize {
  lanternfish(input.to_owned(), 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 5934);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(SAMPLE).unwrap()), 26984457539);
    }
}
