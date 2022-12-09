use std::collections::{HashMap, HashSet, VecDeque};
use scan_fmt::scan_fmt;
use std::io::Write;

type Input = usize;

fn print(board: &Vec<Vec<usize>>, breakpoint: bool) {
    for x in board {
        for y in x {
            print!(".");
        }
        println!();
    }
    println!();

    if breakpoint {
        // Get user input to continue to next step
        std::io::stdout().flush();
        let mut asdf = String::new();
        std::io::stdin().read_line(&mut asdf);
    }
}

#[aoc_generator(dayN)]
pub fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse::<Input>().unwrap());
        //output.push(scan_fmt!(line, "{} {}", char, char).unwrap());
    }
    output
}

#[aoc(dayN, part1)]
pub fn part1(input: &[Input]) -> usize {

    0
}

#[aoc(dayN, part2)]
pub fn part2(input: &[Input]) -> usize {

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
