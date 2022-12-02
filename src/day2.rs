use scan_fmt::scan_fmt;

#[aoc_generator(day2)]
pub fn load_input(input: &str) -> Vec<(char, char)> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(scan_fmt!(line, "{} {}", char, char).unwrap());
    }
    output
}

#[aoc(day2, part1)]
pub fn part1(input: &[(char, char)]) -> u64 {
    let mut score = 0;
    for ll in input {
        score += match ll {
            ('A', 'Y') => 6,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('A', 'X') => 3,
            ('B', 'Y') => 3,
            ('C', 'Z') => 3,
            _ => 0,
        };
        score += match ll.1 {
            'Y' => 2,
            'Z' => 3,
            _ => 1,
        }
    }
    score
}

#[aoc(day2, part2)]
pub fn part2(input: &[(char, char)]) -> u64 {
    let mut score = 0;
    for ll in input {
        score += match ll.1 {
            'X' => 0,
            'Y' => 3,
            _ => 6,
        };
        score += match ll {
            ('A', 'X') => 3,
            ('A', 'Y') => 1,
            ('A', 'Z') => 2,
            ('B', 'X') => 1,
            ('B', 'Y') => 2,
            ('B', 'Z') => 3,
            ('C', 'X') => 2,
            ('C', 'Y') => 3,
            ('C', 'Z') => 1,
            _ => 0,
        };
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 12);
    }
}
