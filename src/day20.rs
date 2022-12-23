use aoc_helpers::linked_list::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::io::stdin;

type Input = i64;

#[aoc_generator(day20)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = Vec::new();
    for line in input.lines() {
        let value = line.trim_end().parse::<i64>().unwrap();
        output.push(value);
    }
    output
}

fn ll_get<T>(list: &LinkedList<T>, idx: usize) -> usize {
    let mut nid = list.head.unwrap();
    for _ in 0..idx {
        nid = list.data[nid].tail.unwrap();
    }
    nid
}

fn ll_find<T: PartialEq>(list: &LinkedList<T>, value: T) -> usize {
    let mut nid = list.head.unwrap();
    loop {
        if list.data[nid].value == value {
            return nid;
        } else {
            nid = list.data[nid].next().unwrap();
        }
    }
}

fn ll_print<T: Debug>(list: &LinkedList<T>) {
    let mut nid = list.head.unwrap();
    let first_id = nid;
    loop {
        print!("{:?} ", list.data[nid].value);
        if let Some(nnid) = list.data[nid].next() {
            nid = nnid;
        }
        if nid == first_id {
            break;
        }
    }
}

// 1467 is too low
#[aoc(day20, part1)]
fn part1(input: &[Input]) -> i64 {
    // Create a linked list
    let mut list = LinkedList::<i64>::with_capacity(16 * input.len());
    let n = input.len();
    for value in input {
        list.push_tail(*value);
    }

    // Make it circular
    let head_id = list.head.unwrap();
    let tail_id = list.tail.unwrap();
    list.data[head_id].head = Some(tail_id);
    list.data[tail_id].tail = Some(head_id);

    for value in input {
        //ll_print(&list);
        let mut nid = ll_find(&list, *value);
        //println!("nid: {}", nid);
        nid = list.data[nid].prev().unwrap();

        let temp = list.pop_id(list.data[nid].next().unwrap());
        //println!("temp: {}", temp);

        // nid currently pointing at the element in list after the one that
        // just got popped.
        if *value > 0 {
            for _ in 0..*value {
                nid = list.data[nid].tail.unwrap();
            }
            list.insert_after(temp, nid);
        } else {
            for _ in 0..(*value).abs() {
                nid = list.data[nid].head.unwrap();
            }
            list.insert_after(temp, nid);
        }
    }

    //ll_print(&list);

    let mut nid = ll_find(&list, 0);
    let mut nums = vec![];

    for _ in 0..3 {
        for _ in 0..1000 {
            nid = list.data[nid].next().unwrap();
        }
        nums.push(list.data[nid].value);
    }
    println!("nums: {:?}", nums);

    nums.iter().sum()
}

#[aoc(day20, part2)]
fn part2(input: &[Input]) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/20.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/20.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
