use scan_fmt::scan_fmt;

type Input = Monkey;

#[derive(Copy, Clone, Debug)]
enum OpArg {
    Old,
    Num(usize),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: char,
    oparg: OpArg,
    divisor: usize,
    truth_id: usize,
    false_id: usize,
}

#[aoc_generator(day11)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    let mut lines = input.lines();
    loop {
        lines.next(); // Monkey #:
        let temp: Vec<_> = lines.next().unwrap().split(": ").collect();
        let items: Vec<_> = temp[1]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (op, num) = scan_fmt!(
            lines.next().unwrap(),
            "  Operation: new = old {} {}",
            char,
            String
        )
        .unwrap();
        let oparg = if num != "old" {
            OpArg::Num(num.parse::<usize>().unwrap())
        } else {
            OpArg::Old
        };
        let divisor = scan_fmt!(lines.next().unwrap(), "  Test: divisible by {}", usize).unwrap();
        let truth_id = scan_fmt!(
            lines.next().unwrap(),
            "    If true: throw to monkey {}",
            usize
        )
        .unwrap();
        let false_id = scan_fmt!(
            lines.next().unwrap(),
            "    If false: throw to monkey {}",
            usize
        )
        .unwrap();
        output.push(Monkey {
            items: items.clone(),
            op,
            oparg,
            divisor,
            truth_id,
            false_id,
        });
        if lines.next().is_none() {
            break;
        }
    }
    output
}

fn monkey_inspection(
    monkey: &Monkey,
    items: &[usize],
    modulo: Option<usize>,
) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for item in items {
        let op_arg: usize = match monkey.oparg {
            OpArg::Old => *item,
            OpArg::Num(nn) => nn,
        };
        let mut new = *item;
        match monkey.op {
            '+' => {
                new += op_arg;
            }
            '*' => new *= op_arg,
            _ => (),
        }

        if let Some(m) = modulo {
            // part 2
            new %= m;
        } else {
            // part 1
            new /= 3;
        }
        let next_monkey = if new % monkey.divisor == 0 {
            monkey.truth_id
        } else {
            monkey.false_id
        };

        output.push((next_monkey, new));
    }
    output
}

#[aoc(day11, part1)]
fn part1(input: &[Input]) -> usize {
    let mut monkey_arr: Vec<Vec<usize>> = vec![vec![]; input.len()];
    for (i, monkey) in input.iter().enumerate() {
        monkey_arr[i] = monkey.items.clone();
    }

    let mut monkeys = vec![0; input.len()];
    for _ in 0..20 {
        for (n, monkey) in input.iter().enumerate() {
            let output = monkey_inspection(monkey, &monkey_arr[n], None);
            monkeys[n] += monkey_arr[n].len();
            monkey_arr[n] = vec![];
            for item in output {
                monkey_arr[item.0].push(item.1);
            }
        }
    }
    monkeys.sort();
    monkeys = monkeys.iter().copied().rev().collect();
    monkeys[0] * monkeys[1]
}

#[aoc(day11, part2)]
fn part2(input: &[Input]) -> usize {
    let mut monkey_arr: Vec<Vec<usize>> = vec![vec![]; input.len()];
    let mut modulo = 1;
    for (i, monkey) in input.iter().enumerate() {
        monkey_arr[i] = monkey.items.clone();
        modulo *= monkey.divisor;
    }
    let mut monkeys = vec![0; input.len()];
    for _ in 0..10000 {
        for (n, monkey) in input.iter().enumerate() {
            let output = monkey_inspection(monkey, &monkey_arr[n], Some(modulo));
            monkeys[n] += monkey_arr[n].len();
            monkey_arr[n] = vec![];
            for item in output {
                monkey_arr[item.0].push(item.1);
            }
        }
    }
    monkeys.sort();
    monkeys = monkeys.iter().copied().rev().collect();
    monkeys[0] * monkeys[1]
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
        assert_eq!(part2(&input), 2713310158);
    }
}
