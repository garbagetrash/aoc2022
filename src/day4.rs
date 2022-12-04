use scan_fmt::scan_fmt;
use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(scan_fmt!(line, "{}-{},{}-{}", i32, i32, i32, i32).unwrap())
    }
    output
}

fn contain_other(e1min: i32, e1max: i32, e2min: i32, e2max: i32) -> bool {
    if e1min <= e2min && e1max >= e2max {
        true
    } else if e2min <= e1min && e2max >= e1max {
        true
    } else {
        false
    }
}

fn overlap_at_all(e1min: i32, e1max: i32, e2min: i32, e2max: i32) -> bool {
    let mut range1 = HashSet::new();
    for i in e1min..e1max + 1 {
        range1.insert(i);
    }
    let mut range2 = HashSet::new();
    for i in e2min..e2max + 1 {
        range2.insert(i);
    }
    let thing: Vec<_> = range1.intersection(&range2).collect();
    if thing.len() >= 1 {
        true
    } else {
        false
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &[(i32, i32, i32, i32)]) -> u32 {
    let mut cntr = 0;
    for line in input {
        if contain_other(line.0, line.1, line.2, line.3) {
            cntr += 1;
        }
    }
    cntr
}

#[aoc(day4, part2)]
pub fn part2(input: &[(i32, i32, i32, i32)]) -> u32 {
    let mut cntr = 0;
    for line in input {
        if overlap_at_all(line.0, line.1, line.2, line.3) {
            cntr += 1;
        }
    }
    cntr
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/04a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/04a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 4);
    }
}
