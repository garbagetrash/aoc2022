use scan_fmt::scan_fmt;
use std::collections::VecDeque;

static SETUP: [char; 56] = [
    'B', 'S', 'V', 'Z', 'G', 'P', 'W', // 7
    'J', 'V', 'B', 'C', 'Z', 'F', // 6
    'V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C', // 8
    'L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B', // 8
    'V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H', // 8
    'G', 'F', 'Q', 'T', 'S', 'L', 'B', // 7
    'L', 'G', 'C', 'Z', 'V', // 5
    'N', 'L', 'G', // 3
    'J', 'F', 'H', 'C', // 4
];

#[aoc_generator(day5)]
fn load_input(input: &str) -> Vec<(usize, usize, usize)> {
    let mut start = true;
    let mut output = vec![];
    for line in input.lines() {
        if !start {
            output.push(scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap())
        }
        if line == "" {
            start = false;
        }
    }
    output
}

fn get_setup() -> Vec<VecDeque<char>> {
    let mut output = vec![];
    output.push(SETUP.iter().copied().take(7).collect());
    output.push(SETUP.iter().copied().skip(7).take(6).collect());
    output.push(SETUP.iter().copied().skip(13).take(8).collect());
    output.push(SETUP.iter().copied().skip(21).take(8).collect());
    output.push(SETUP.iter().copied().skip(29).take(8).collect());
    output.push(SETUP.iter().copied().skip(37).take(7).collect());
    output.push(SETUP.iter().copied().skip(44).take(5).collect());
    output.push(SETUP.iter().copied().skip(49).take(3).collect());
    output.push(SETUP.iter().copied().skip(52).take(4).collect());

    output
}

#[aoc(day5, part1)]
pub fn part1(input: &[(usize, usize, usize)]) -> String {
    let mut setup = get_setup();
    for line in input {
        let (n_move, mut from, mut to) = line;
        from -= 1;
        to -= 1;
        for _ in 0..*n_move {
            let temp = setup[from].pop_back().unwrap();
            setup[to].push_back(temp);
        }
    }

    let mut output = String::new();
    for i in 0..9 {
        output.push(setup[i].pop_back().unwrap());
    }
    output
}

#[aoc(day5, part2)]
pub fn part2(input: &[(usize, usize, usize)]) -> String {
    let mut setup = get_setup();
    for line in input {
        let (n_move, mut from, mut to) = line;
        from -= 1;
        to -= 1;
        let value = setup[from].len() - *n_move;
        let mut temp = setup[from].split_off(value);
        setup[to].append(&mut temp);
    }

    let mut output = String::new();
    for i in 0..9 {
        output.push(setup[i].pop_back().unwrap());
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/05.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/05.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), String::from("MCD"));
    }
}
