use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Atom {
    List(Vec<Atom>),
    Number(usize),
    Right,
    Left,
    Comma,
}

fn simplify_list(tlist: &[Atom]) -> Atom {
    let mut output = vec![];
    for t in tlist {
        match t {
            Atom::Number(n) => output.push(Atom::Number(*n)),
            Atom::List(x) => output.push(Atom::List(x.clone())),
            _ => (),
        }
    }

    Atom::List(output)
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(b) = self.compare(other) {
            if b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(b) = self.compare(other) {
            if b {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Atom {
    #[allow(clippy::unnecessary_unwrap)]
    fn compare(&self, right: &Atom) -> Option<bool> {
        match (self, right) {
            (Atom::List(l), Atom::List(r)) => {
                let mut ll = l.iter();
                let mut rr = r.iter();

                loop {
                    let lval = ll.next();
                    let rval = rr.next();
                    if lval.is_none() && rval.is_none() {
                        return None;
                    } else if lval.is_none() {
                        return Some(true);
                    } else if rval.is_none() {
                        return Some(false);
                    } else if let Some(answer) = (lval.unwrap()).compare(rval.unwrap()) {
                        return Some(answer);
                    } else {
                        // Try the next value
                        continue;
                    }
                }
            }
            (Atom::Number(l), Atom::Number(r)) => {
                if l == r {
                    None
                } else {
                    Some(l < r)
                }
            }
            (Atom::List(llist), Atom::Number(r)) => {
                Atom::List(llist.to_vec()).compare(&Atom::List(vec![Atom::Number(*r)]))
            }
            (Atom::Number(l), Atom::List(rlist)) => {
                Atom::List(vec![Atom::Number(*l)]).compare(&Atom::List(rlist.to_vec()))
            }
            _ => None,
        }
    }
}

fn parse_string(s: &str) -> Atom {
    let mut buildnum = String::new();
    let mut tokens = vec![];
    for ss in s.chars() {
        match ss {
            '[' => tokens.push(Atom::Left),
            ']' => {
                if !buildnum.is_empty() {
                    tokens.push(Atom::Number(buildnum.parse::<usize>().unwrap()));
                    buildnum = String::new();
                }
                tokens.push(Atom::Right);
            }
            ',' => {
                if !buildnum.is_empty() {
                    tokens.push(Atom::Number(buildnum.parse::<usize>().unwrap()));
                    buildnum = String::new();
                }
                tokens.push(Atom::Comma);
            }
            _ => buildnum.push(ss),
        }
    }

    // Parse Atoms into Atoms
    let mut stack: VecDeque<Atom> = VecDeque::new();
    for t in tokens {
        match t {
            Atom::Right => {
                let mut temp = vec![];
                loop {
                    let value = stack.pop_back().unwrap();
                    if value == Atom::Left {
                        break;
                    } else {
                        temp.push(value);
                    }
                }
                let v: Vec<Atom> = temp.into_iter().rev().collect();
                let a: Atom = simplify_list(&v);
                stack.push_back(a);
            }
            _ => stack.push_back(t),
        }
    }
    if stack.len() > 1 {
        panic!("stack didn't work");
    }
    stack[0].clone()
}

#[derive(Clone, Debug, PartialEq)]
struct PacketPair {
    p1: String,
    p2: String,
}

impl PacketPair {
    fn compare(&self) -> bool {
        let left = parse_string(&self.p1);
        let right = parse_string(&self.p2);
        left.compare(&right).unwrap()
    }
}

type Input = PacketPair;

#[aoc_generator(day13)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    let mut line = input.lines();

    loop {
        let p1 = String::from(line.next().unwrap());
        let p2 = String::from(line.next().unwrap());
        output.push(PacketPair { p1, p2 });
        if line.next().is_none() {
            break;
        }
    }
    output
}

#[aoc(day13, part1)]
fn part1(input: &[Input]) -> usize {
    let mut idxs = vec![];
    for (i, line) in input.iter().enumerate() {
        if line.compare() {
            idxs.push(i + 1);
        }
    }
    idxs.iter().sum::<usize>()
}

#[aoc(day13, part2)]
fn part2(input: &[Input]) -> usize {
    let mut lines = vec![];
    for pair in input {
        lines.push(pair.p1.clone());
        lines.push(pair.p2.clone());
    }
    lines.push(String::from("[[2]]"));
    lines.push(String::from("[[6]]"));

    let mut atoms = vec![];
    for line in lines {
        let atom = parse_string(&line);
        atoms.push(atom);
    }
    let d1 = atoms[atoms.len() - 1].clone();
    let d2 = atoms[atoms.len() - 2].clone();
    atoms.sort();

    let mut idxs = vec![];
    for (i, atom) in atoms.iter().enumerate() {
        if *atom == d1 || *atom == d2 {
            idxs.push(i + 1);
        }
    }
    idxs[0] * idxs[1]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/13.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/13.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 140);
    }

    #[test]
    fn test_chris() {
        let input = read_to_string("input/2022/chris_day13.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5760);
    }
}
