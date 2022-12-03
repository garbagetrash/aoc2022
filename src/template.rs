use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;
use scan_fmt::scan_fmt;

#[aoc_generator(dayNN)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse::<u64>().unwrap());
        //output.push(scan_fmt!(line, "{} {}", char, char).unwrap());
    }
    output
}

#[aoc(dayNN, part1)]
pub fn part1(input: &[u64]) -> u64 {

    0
}

#[aoc(dayNN, part2)]
pub fn part2(input: &[u64]) -> u64 {

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/XXa.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/XXa.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
