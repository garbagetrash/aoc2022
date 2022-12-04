use scan_fmt::scan_fmt;

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(scan_fmt!(line, "{}-{},{}-{}", i32, i32, i32, i32).unwrap())
    }
    output
}

fn contain_other(e1min: i32, e1max: i32, e2min: i32, e2max: i32) -> bool {
    (e1min <= e2min && e1max >= e2max) || (e2min <= e1min && e2max >= e1max)
}

fn overlap_at_all(e1min: i32, e1max: i32, e2min: i32, e2max: i32) -> bool {
    !(e1max < e2min || e2max < e1min)
}

#[aoc(day4, part1)]
pub fn part1(input: &[(i32, i32, i32, i32)]) -> usize {
    input
        .iter()
        .filter(|line| contain_other(line.0, line.1, line.2, line.3))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(i32, i32, i32, i32)]) -> usize {
    input
        .iter()
        .filter(|line| overlap_at_all(line.0, line.1, line.2, line.3))
        .count()
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
