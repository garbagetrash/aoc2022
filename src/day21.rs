use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::stdin;


#[derive(Clone, Copy, Debug, PartialEq)]
enum Atom {
    Value(i64),
    Symbol(char),
    Op(char),
}

#[derive(Clone, Debug, PartialEq)]
enum Expression {
    Atom,
    Expr((Expression, Atom, Expression)),
}

impl Expression {
    fn eval(&self, environment: &HashMap<String, Expression>) -> Option<Expression> {
        println!("eval: {:?}", self);
        match self {
            Expression::Expr((str1_expr, op, str2_expr)) => {
                let str1_expr = environment.get(str1_expr).unwrap();
                let str2_expr = environment.get(str2_expr).unwrap();
                match str1_expr {
                    Expression::Value(v1) => {
                        match str2_expr {
                            Expression::Value(v2) => {
                                match op {
                                    '+' => return Some(Expression::Value(v1 + v2)),
                                    '-' => return Some(Expression::Value(v1 - v2)),
                                    '*' => return Some(Expression::Value(v1 * v2)),
                                    '/' => return Some(Expression::Value(v1 / v2)),
                                    _ => (),
                                }
                            },
                            bol(c) => {
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        //println!("  Expression did not resolve");
        Some(self.clone())
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        let words: Vec<_> = value.split(' ').collect();
        if words.len() == 1 {
            // Just a value
            return Expression::Value(words[0].parse::<i64>().unwrap());
        }
        let str1 = words[0].to_string();
        let op = words[1].chars().next().unwrap();
        let str2 = words[2].to_string();
        Expression::Unresolved((str1, op, str2))
    }
}

type Input = HashMap<String, Expression>;

#[aoc_generator(day21)]
fn load_input(input: &str) -> Input {
    let mut output = HashMap::new();
    for line in input.lines() {
        // (bnum, orebot, claybot, obsbot_ore, obsbot_clay, geobot_ore, geobot_obs)
        let words: Vec<_> = line.trim_end().split(": ").collect();
        let var = words[0].to_string();
        let expr = words[1];
        output.insert(var, Expression::from(expr));
    }
    output
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> i64 {
    let mut env = input.clone();
    let keys: Vec<String> = env.keys().cloned().collect();
    loop {
        //println!("\nenv: {:?}", env);
        for key in &keys {
            //println!("  key: {:?}", key);
            let mut new_expr = None;
            let expr = env.get(key).unwrap();
            match expr {
                Expression::Unresolved((str1, op, str2)) => {
                    new_expr = expr.eval(&env);
                },
                _ => (),//println!("  already {:?}", expr),
            };
            if let Some(new) = new_expr {
                //println!("  resolved to {:?}", new);
                env.insert(key.to_string(), new);
            }
        }

        // If no expression are Expression::Unresolved anymore, bail
        if !env.iter().any(|x| match *x.1 {
            Expression::Unresolved(_) => true,
            _ => false,
        }) {
            break;
        }
    }
    match env.get("root").unwrap() {
        Expression::Value(answer) => return *answer,
        _ => unreachable!(),
    }
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> i64 {
    let mut env = input.clone();
    env.insert("humn".to_string(), Atom::Symbol('X'));
    let keys: Vec<String> = env.keys().cloned().collect();
    loop {
        println!("\nenv: {:?}", env);
        for key in &keys {
            //println!("  key: {:?}", key);
            if key == "root" || key == "humn" {
                continue;
            }
            let mut new_expr = None;
            let expr = env.get(key).unwrap();
            match expr {
                Expression::Unresolved((str1, op, str2)) => {
                    new_expr = expr.eval(&env);
                },
                _ => (),//println!("  already {:?}", expr),
            };
            if let Some(new) = new_expr {
                //println!("  resolved to {:?}", new);
                env.insert(key.to_string(), new);
            }
        }

        // If no expression are Expression::Unresolved anymore, bail
        if !env.iter().filter(|x| match *x.1 {
            Expression::Unresolved(_) => true,
            _ => false,
        }).count() == 0 {
            break;
        }
    }
    match env.get("root").unwrap() {
        Expression::Value(answer) => return *answer,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 150);
    }
}
