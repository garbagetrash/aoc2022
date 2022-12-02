use regex::Regex;
use scan_fmt::{scan_fmt, scan_fmt_some};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Z = u64;

#[aoc_generator(day1)]
pub fn load_input(input: &str) -> Vec<Z> {
    let mut sums: Vec<Z> = vec![];
    let mut thissum: Z = 0;
    for line in input.lines() {
        //println!("{}", line);
        if line == "" {
            sums.push(thissum);
            thissum = 0;
        } else {
            thissum += line.parse::<Z>().unwrap();
        }
    }
    sums
}

#[aoc(day01, part1)]
pub fn part1(input: &[Z]) -> Z {
    *input.iter().max().unwrap()
}

#[aoc(day01, part2)]
pub fn part2(input: &[Z]) -> Z {
    let mut temp: Vec<_> = input.iter().copied().collect();
    temp.sort();
    temp.iter().rev().take(3).sum::<Z>()
}

#[cfg(test)]
mod test {
    use super::*;
    use scan_fmt::{scan_fmt, scan_fmt_some};
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 45000);
    }

    #[test]
    fn test_scan_fmt() {
        let (a, b, c) = scan_fmt!("hello 0x12 345 bye",            // input
                                  "hello {x} {} {}",               // format
                                  [hex u8], i32, String)
        .unwrap(); // types
        assert_eq!(a, 0x12);
        assert_eq!(b, 345);
        assert_eq!(c, "bye");

        /* Gets user input
        let (c, d) = scanln_fmt!("{d}--{d}",         // format
                                  u16, u8).unwrap(); // type
        */

        let (a, b) = scan_fmt_some!(
            "hello 12 345", // input
            "hello {} {}",  // format
            u8,
            i32
        ); // types
        assert_eq!(a, Some(12));
        assert_eq!(b, Some(345));
    }
}
