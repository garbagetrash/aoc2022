use scan_fmt::scan_fmt;

struct Input {
    setup: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

#[aoc_generator(day5)]
fn load_input(input: &str) -> Input {
    let mut start = true;
    let mut instructions = vec![];
    let mut start_end_idx = 0;
    for (i, line) in input.lines().enumerate() {
        if !start {
            instructions
                .push(scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap())
        }
        if line.is_empty() {
            start = false;
            start_end_idx = i;
        }
    }

    let peek: String = input.lines().take(1).collect();
    let n_cols = (peek.len() + 1) / 4;
    let mut setup: Vec<Vec<char>> = vec![vec![]; n_cols];
    let start_lines: Vec<_> = input.lines().take(start_end_idx).collect();
    for line in start_lines.into_iter().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                setup[i].push(c);
            }
        }
    }
    Input {
        setup,
        instructions,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> String {
    let mut setup = input.setup.clone();
    for (n_move, from, to) in &input.instructions {
        for _ in 0..*n_move {
            let temp = setup[from - 1].pop().unwrap();
            setup[to - 1].push(temp);
        }
    }

    let mut output = String::new();
    for mut col in setup {
        output.push(col.pop().unwrap());
    }
    output
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> String {
    let mut setup = input.setup.clone();
    for (n_move, from, to) in &input.instructions {
        let value = setup[from - 1].len() - n_move;
        let mut temp = setup[from - 1].split_off(value);
        setup[to - 1].append(&mut temp);
    }

    let mut output = String::new();
    for mut col in setup {
        output.push(col.pop().unwrap());
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
