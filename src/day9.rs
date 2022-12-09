#![allow(clippy::comparison_chain)]
use scan_fmt::scan_fmt;
use std::collections::HashSet;

type Input = Vec<(char, usize)>;

#[aoc_generator(day9)]
fn load_input(input: &str) -> Input {
    let mut output = vec![];
    for line in input.lines() {
        let (dir, n) = scan_fmt!(line, "{} {}", char, usize).unwrap();
        output.push((dir, n));
    }
    output
}

fn move_towards(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let xdist = (tail.0 - head.0).abs();
    let ydist = (tail.1 - head.1).abs();
    let mdist = xdist + ydist;
    let mut m = (0, 0);
    if mdist > 2 {
        // diag
        if tail.0 - head.0 > 0 {
            m.0 = -1;
        } else if tail.0 - head.0 < 0 {
            m.0 = 1;
        }
        if tail.1 - head.1 > 0 {
            m.1 = -1;
        } else if tail.1 - head.1 < 0 {
            m.1 = 1;
        }
    } else {
        if tail.0 - head.0 > 1 {
            m.0 = -1;
        } else if tail.0 - head.0 < -1 {
            m.0 = 1;
        }
        if tail.1 - head.1 > 1 {
            m.1 = -1;
        } else if tail.1 - head.1 < -1 {
            m.1 = 1;
        }
    }

    (tail.0 + m.0, tail.1 + m.1)
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> usize {
    let mut tail_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    tail_locs.insert(tail);
    for line in input {
        for _ in 0..line.1 {
            match line.0 {
                'R' => {
                    head.1 += 1;
                    tail = move_towards(tail, head);
                }
                'D' => {
                    head.0 -= 1;
                    tail = move_towards(tail, head);
                }
                'L' => {
                    head.1 -= 1;
                    tail = move_towards(tail, head);
                }
                'U' => {
                    head.0 += 1;
                    tail = move_towards(tail, head);
                }
                _ => (),
            }
            tail_locs.insert(tail);
        }
    }
    tail_locs.len()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> usize {
    let mut tail_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];
    tail_locs.insert(head);
    for line in input {
        for _ in 0..line.1 {
            match line.0 {
                'R' => {
                    head.1 += 1;
                    tails[0] = move_towards(tails[0], head);
                    for i in 1..9 {
                        tails[i] = move_towards(tails[i], tails[i - 1]);
                    }
                }
                'D' => {
                    head.0 -= 1;
                    tails[0] = move_towards(tails[0], head);
                    for i in 1..9 {
                        tails[i] = move_towards(tails[i], tails[i - 1]);
                    }
                }
                'L' => {
                    head.1 -= 1;
                    tails[0] = move_towards(tails[0], head);
                    for i in 1..9 {
                        tails[i] = move_towards(tails[i], tails[i - 1]);
                    }
                }
                'U' => {
                    head.0 += 1;
                    tails[0] = move_towards(tails[0], head);
                    for i in 1..9 {
                        tails[i] = move_towards(tails[i], tails[i - 1]);
                    }
                }
                _ => (),
            }
            tail_locs.insert(tails[8]);
        }
    }
    tail_locs.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = load_input(&read_to_string("input/2022/09.txt").unwrap());
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = load_input(&read_to_string("input/2022/09.txt").unwrap());
        assert_eq!(part2(&input), 1);
        let input = load_input(&read_to_string("input/2022/09a.txt").unwrap());
        assert_eq!(part2(&input), 36);
    }
}
