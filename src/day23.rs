use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::stdin;
use std::{thread, time};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Input = HashSet<(i64, i64)>;

#[aoc_generator(day23)]
fn load_input(input: &str) -> Input {
    let mut output = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    output.insert((row as i64, col as i64));
                }
                _ => (),
            }
        }
    }
    output
}

fn get_neighbors(p: (i64, i64)) -> Vec<(i64, i64)> {
    let mut output = vec![];

    // NW, N, NE = 0..3
    output.push((p.0 - 1, p.1 - 1));
    output.push((p.0 - 1, p.1));
    // NE, E, SE = 2..5
    output.push((p.0 - 1, p.1 + 1));
    output.push((p.0, p.1 + 1));
    // SE, S, SW = 4..7
    output.push((p.0 + 1, p.1 + 1));
    output.push((p.0 + 1, p.1));
    // SW, W, NW = 6..8, 0
    output.push((p.0 + 1, p.1 - 1));
    output.push((p.0, p.1 - 1));

    output
}

fn next_direction(p: (i64, i64), round_idx: usize, map: &Input) -> Option<Direction> {
    let n = get_neighbors(p);
    let n_idx = vec![0_usize, 1, 2];
    let s_idx = vec![4_usize, 5, 6];
    let w_idx = vec![0_usize, 6, 7];
    let e_idx = vec![2_usize, 3, 4];

    let m = round_idx % 4;
    let mut c_idxs = vec![
        (n_idx, Direction::North),
        (s_idx, Direction::South),
        (w_idx, Direction::West),
        (e_idx, Direction::East),
    ];

    if n.iter().all(|x| !map.contains(x)) {
        // If all neighbors do not contain an Elf, do nothing.
        None
    } else {
        if c_idxs[m % 4].0.iter().map(|&i| n[i]).all(|x| !map.contains(&x)) {
            // If all x options do not contain an Elf, go x
            Some(c_idxs[m % 4].1)
        } else if c_idxs[(m + 1) % 4].0.iter().map(|&i| n[i]).all(|x| !map.contains(&x)) {
            // If all x options do not contain an Elf, go x
            Some(c_idxs[(m + 1) % 4].1)
        } else if c_idxs[(m + 2) % 4].0.iter().map(|&i| n[i]).all(|x| !map.contains(&x)) {
            // If all x options do not contain an Elf, go x
            Some(c_idxs[(m + 2) % 4].1)
        } else if c_idxs[(m + 3) % 4].0.iter().map(|&i| n[i]).all(|x| !map.contains(&x)) {
            // If all x options do not contain an Elf, go x
            Some(c_idxs[(m + 3) % 4].1)
        } else {
            // Too many elves around, just stay where you are
            None
        }
    }
}

fn next_pos(orig_pos: (i64, i64), dir: Option<Direction>) -> (i64, i64) {
    match dir {
        None => orig_pos,
        Some(Direction::North) => (orig_pos.0 - 1, orig_pos.1),
        Some(Direction::East) => (orig_pos.0, orig_pos.1 + 1),
        Some(Direction::South) => (orig_pos.0 + 1, orig_pos.1),
        Some(Direction::West) => (orig_pos.0, orig_pos.1 - 1),
    }
}

fn round(map: &Input, round_idx: usize) -> Input {
    // First half
    let mut next_dirs: Vec<((i64, i64), Option<Direction>)> = vec![];
    for orig_pos in map {
        next_dirs.push((*orig_pos, next_direction(*orig_pos, round_idx, map)));
    }
    //println!("{:?}", next_dirs);

    // Second half

    // First figure out where everybody wants to go
    let mut candidate_map = HashMap::new();
    for (orig_pos, dir) in &next_dirs {
        let np = next_pos(*orig_pos, *dir);
        *candidate_map.entry(np).or_insert(0) += 1;
    }
    //println!("\n{:?}", candidate_map);
    let mut cancelled_set = HashSet::new();
    for (k, v) in candidate_map {
        // Cancel moves that would lead to collisions
        if v > 1 {
            cancelled_set.insert(k);
        }
    }
    //println!("\n{:?}", cancelled_set);

    // Execute moves if not in cancelled set
    let mut next_map = HashSet::new();
    for (orig_pos, dir) in next_dirs {
        let np = next_pos(orig_pos, dir);
        if cancelled_set.contains(&np) {
            // This was cancelled
            next_map.insert(orig_pos);
        } else {
            next_map.insert(np);
        }
    }
    next_map
}

fn get_map_extent(map: &Input) -> (i64, i64, i64, i64) {
    // Find extent of the map
    let mut minx = i64::MAX;
    let mut miny = i64::MAX;
    let mut maxx = i64::MIN;
    let mut maxy = i64::MIN;
    for p in map {
        if p.0 > maxy {
            maxy = p.0;
        } else if p.0 < miny {
            miny = p.0;
        }
        if p.1 > maxx {
            maxx = p.1;
        } else if p.1 < minx {
            minx = p.1;
        }
    }
    (minx, maxx, miny, maxy)
}

fn draw_map(map: &Input) {

    let (minx, maxx, miny, maxy) = get_map_extent(map);

    // To let test garbage printing finish up before draw
    thread::sleep(time::Duration::from_millis(10));

    // Draw it
    println!();
    for row in miny..maxy + 1 {
        for col in minx..maxx + 1 {
            if map.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[aoc(day23, part1)]
fn part1(input: &Input) -> i64 {
    let mut map = input.clone();
    for i in 0..11 {
        //draw_map(&map);
        map = round(&map, i);
    }
    let (minx, maxx, miny, maxy) = get_map_extent(&map);
    let total = (maxx - minx + 1) * (maxy - miny + 1);
    total - map.len() as i64
}

#[aoc(day23, part2)]
fn part2(input: &Input) -> usize {
    let mut map = input.clone();
    let mut cntr = 0;
    loop {
        //draw_map(&map);
        let last_map = map.clone();
        map = round(&map, cntr);
        cntr += 1;

        if map == last_map {
            break;
        }
    }
    cntr
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/23.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/23.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 20);
    }
}
