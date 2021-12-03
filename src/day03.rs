use anyhow::Result;
use aoc_runner_derive::aoc;

// shamelessly copied from https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn gamma_bit(bits: &[char]) -> char {
    let (zeroes, ones): (Vec<&char>, Vec<&char>) = bits.iter().partition(|b| **b == '0');
    if zeroes.len() > ones.len() { '0' } else { '1' }
}

fn invert(bits: &str) -> String {
    bits.chars().map(|b| if b == '0' { '1' } else { '0' }).collect()
}

fn gamma(readings: Vec<Vec<char>>) -> Result<u32> {
    let gamma_str: String = transpose(readings)
        .into_iter()
        .map(|column| gamma_bit(&column))
        .collect();

    u32::from_str_radix(&gamma_str, 2).map_err(Into::into)
}

fn epsilon(readings: Vec<Vec<char>>) -> Result<u32> {
    let gamma_str: String = transpose(readings)
        .into_iter()
        .map(|column| gamma_bit(&column))
        .collect();

    let epsilon_str = invert(&gamma_str);
    u32::from_str_radix(&epsilon_str, 2).map_err(Into::into)
}

#[aoc(day3, part1)]
fn part1(input: &str) -> Result<u64> {
    let input_chars: Vec<Vec<char>> = input.lines().map(str::chars).map(Iterator::collect).collect();    
    
    let gamma = gamma(input_chars.clone())?;
    let epsilon = epsilon(input_chars)?;
    
    Ok(gamma as u64 * epsilon as u64)
}

fn reading_column(readings: &[String], index: usize) -> Vec<char> {
    readings.iter().map(|reading| reading.chars().nth(index).unwrap()).collect()
}

fn bit_counts(readings: &[String], index: usize) -> (usize, usize) {
    let (zeroes, ones): (Vec<char>, Vec<char>) = reading_column(readings, index).iter().partition(|bit| **bit == '0');
    (zeroes.len(), ones.len())
}

fn most_common_bit(readings: &[String], index: usize) -> char {
    let (zeroes, ones) = bit_counts(readings, index);
    if zeroes > ones { '0' } else { '1' }
}

fn least_common_bit(readings: &[String], index: usize) -> char {
    let (zeroes, ones) = bit_counts(readings, index);
    if zeroes <= ones { '0' } else { '1' }
}

fn filter_readings(readings: Vec<String>, index: usize, filter_bit: char) -> Vec<String> {
    readings
        .into_iter()
        .filter(|reading| {
            reading.chars().nth(index).unwrap() == filter_bit
        })
        .collect()
}

fn oxygen_generator(readings: Vec<String>) -> Result<u32> {
    let mut candidates = readings.clone();

    for i in 0.. {
        let mcb = most_common_bit(&candidates, i);
        
        candidates = filter_readings(candidates, i, mcb);
        
        if candidates.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&candidates[0], 2).map_err(Into::into)
}

fn co2_scrubber(readings: Vec<String>) -> Result<u32> {
    let mut candidates = readings.clone();

    for i in 0.. {
        let lcb = least_common_bit(&candidates, i);
        
        candidates = filter_readings(candidates, i, lcb);
        
        if candidates.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&candidates[0], 2).map_err(Into::into)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> Result<u64> {
    let input_chars: Vec<String> = input.lines().map(ToOwned::to_owned).collect();
    let oxygen_generator = oxygen_generator(input_chars.clone())?;
    let co2_scrubber = co2_scrubber(input_chars)?;

    Ok(oxygen_generator as u64 * co2_scrubber as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = 
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn sample1() {
        assert_eq!(part1(SAMPLE).unwrap(), 198);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(SAMPLE).unwrap(), 230);
    }
}
