use scan_fmt::scan_fmt;

#[derive(Debug, Copy, Clone)]
enum Command {
    Addx(i64),
    Noop,
}

type Input = Poop;

#[derive(Copy, Clone, Debug)]
enum OpArg {
    Old,
    Num(usize),
}

#[derive(Debug)]
struct Poop {
    mnum: usize,
    sitems: Vec<usize>,
    op: char,
    oparg: OpArg,
    divby: usize,
    mtruth: usize,
    mfalse: usize,
}

#[aoc_generator(day11)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    let mut i = 0;
    let mut mnum = 0;
    let mut sitems: Vec<usize> = vec![];
    let mut n_op = '+';
    let mut oparg = OpArg::Old;
    let mut divby = 0;
    let mut truth_monkey = 0;
    let mut false_monkey = 0;
    for line in input.lines() {
        match i % 7 {
            0 => {
                mnum = scan_fmt!(line, "Monkey {}:", usize).unwrap();
            }
            1 => {
                let asdf: Vec<_> = line.split(": ").collect();
                sitems = asdf[1]
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
            }
            2 => {
                let (op, num) =
                    scan_fmt!(line, "  Operation: new = old {} {}", char, String).unwrap();
                n_op = op;
                println!("derp: {}", num);
                if num != "old" {
                    oparg = OpArg::Num(num.parse::<usize>().unwrap());
                } else {
                    oparg = OpArg::Old;
                }
            }
            3 => {
                divby = scan_fmt!(line, "  Test: divisible by {}", usize).unwrap();
            }
            4 => {
                truth_monkey = scan_fmt!(line, "    If true: throw to monkey {}", usize).unwrap();
            }
            5 => {
                false_monkey = scan_fmt!(line, "    If false: throw to monkey {}", usize).unwrap();
                output.push(Poop {
                    mnum: mnum,
                    sitems: sitems.clone(),
                    op: n_op,
                    oparg: oparg,
                    divby: divby,
                    mtruth: truth_monkey,
                    mfalse: false_monkey,
                });
            }
            6 => {
            },
            _ => (),
        }
        i += 1;
    }
    output
}

#[aoc(day11, part1)]
fn part1(input: &[Input]) -> i64 {
    let mut arr: Vec<Vec<usize>> = vec![vec![]; input.len()];
    for (i, line) in input.iter().enumerate() {
        println!("line: {:?}", line);
        arr[i] = line.sitems.clone();
    }

    println!("lenght: {}", input.len());
    let mut monkeys = vec![0; input.len()];
    for _ in 0..20 {
        let mut next_arr: Vec<Vec<usize>> = vec![vec![]; monkeys.len()];
        for (n, line) in input.iter().enumerate() {
            println!("{:?}", line);
            println!("items: {:?}", arr[n]);

            for item in &arr[n] {
                let op_arg: usize = match line.oparg {
                    OpArg::Old => *item,
                    OpArg::Num(nn) => nn,
                };
                let mut new = *item;
                println!("new: {}, op_arg: {}", new, op_arg);
                match line.op {
                    '+' => {
                        new += op_arg;
                    },
                    '*' => new *= op_arg,
                    _ => (),
                }

                let mut next_monkey = 0;
                new /= 3;
                if new % line.divby == 0 {
                    next_monkey = line.mtruth;
                } else {
                    next_monkey = line.mfalse;
                }

                println!("  item value {} is thrown to monkey {}", new, next_monkey);
                next_arr[next_monkey].push(new);
                monkeys[n] += 1;
            }
        }
        arr = next_arr;

        for m in &monkeys {
            println!("  monkey: {:?}", m);
        }
        for items in &arr {
            println!("  items: {:?}", items);
        }
        println!();
    }
    monkeys.sort();
    monkeys = monkeys.iter().copied().rev().collect();
    monkeys[0] * monkeys[1]
}

#[aoc(day11, part2)]
fn part2(input: &[Input]) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/11.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/11.txt").unwrap();
        let input = load_input(&input);
        part2(&input);
        assert_eq!(0, 0);
    }
}
