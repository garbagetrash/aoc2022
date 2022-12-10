#[derive(Debug, Copy, Clone)]
pub enum Command {
    Addx(i64),
    Noop,
}

type Input = Command;

#[aoc_generator(day10)]
pub fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        match words[0] {
            "addx" => {
                output.push(Command::Addx(words[1].parse::<i64>().unwrap()));
            }
            "noop" => {
                output.push(Command::Noop);
            }
            _ => (),
        }
    }
    output
}

#[aoc(day10, part1)]
pub fn part1(input: &[Input]) -> i64 {
    let mut x = 1;
    let mut i = 1;
    let mut sum = 0;
    for inst in input {
        match inst {
            Command::Noop => {
                if (i + 20) % 40 == 0 {
                    sum += i * x;
                }
                i += 1;
            }
            Command::Addx(v) => {
                if (i + 20) % 40 == 0 {
                    sum += i * x;
                }
                i += 1;
                if (i + 20) % 40 == 0 {
                    sum += i * x;
                }
                i += 1;
                x += v;
            }
        }
    }
    sum
}

fn draw(i: usize, x: i64, image: &mut [[char; 40]; 6]) {
    let yp: usize = (i - 1) / 40;
    let xp: usize = (i - 1) % 40;
    if (x - xp as i64).abs() < 2 {
        // draw
        if xp < 40 {
            image[yp][xp] = '#';
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[Input]) -> i64 {
    let mut x: i64 = 1;
    let mut i = 1;
    let mut image = [['.'; 40]; 6];
    for inst in input {
        draw(i, x, &mut image);
        match inst {
            Command::Noop => {
                i += 1;
            }
            Command::Addx(v) => {
                i += 1;
                draw(i, x, &mut image);
                i += 1;
                x += v;
            }
        }
    }

    for line in image {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/10.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/10.txt").unwrap();
        let input = load_input(&input);
        part2(&input);
        assert_eq!(0, 0);
    }
}
