use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Start,
    End,
    Value(i32),
}

struct HeightMap {
    map: Vec<Vec<Tile>>,
}

impl HeightMap {
    fn start_pos(&self) -> (usize, usize) {
        for (row, line) in self.map.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if *tile == Tile::Start {
                    return (col, row);
                }
            }
        }
        unreachable!()
    }
    fn end_pos(&self) -> (usize, usize) {
        for (row, line) in self.map.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if *tile == Tile::End {
                    return (col, row);
                }
            }
        }
        unreachable!()
    }
    fn height(&self, pos: (usize, usize)) -> i32 {
        match self.map[pos.1][pos.0] {
            Tile::Start => 0,
            Tile::End => 25,
            Tile::Value(num) => num,
        }
    }
}

type Input = HeightMap;

// let input_iter = input.iter();
// loop {
//     let line = input_iter.next().unwrap();
// }
//
// let comma_separated = line.split(',').collect();

#[aoc_generator(day12)]
fn load_input(input: &str) -> Input {
    let mut output = HeightMap { map: vec![] };
    for line in input.lines() {
        let mut temp = vec![];
        for c in line.chars() {
            match c {
                'S' => {
                    temp.push(Tile::Start);
                }
                'E' => {
                    temp.push(Tile::End);
                }
                _ => {
                    let cc = (c as i32) - 97;
                    temp.push(Tile::Value(cc));
                }
            }
        }
        output.map.push(temp);
    }
    output
}

fn neighbors(pos: (usize, usize), map: &HeightMap) -> Vec<(usize, usize)> {
    let w = map.map[0].len();
    let h = map.map.len();
    let height = map.height(pos);
    let mut output = vec![];
    //up
    if pos.1 > 0 {
        let other = (pos.0, pos.1 - 1);
        let hother = map.height(other);
        if hother <= height + 1 {
            output.push(other);
        }
    }
    //down
    if pos.1 < h - 1 {
        let other = (pos.0, pos.1 + 1);
        let hother = map.height(other);
        if hother <= height + 1 {
            output.push(other);
        }
    }
    //left
    if pos.0 > 0 {
        let other = (pos.0 - 1, pos.1);
        let hother = map.height(other);
        if hother <= height + 1 {
            output.push(other);
        }
    }
    //right
    if pos.0 < w - 1 {
        let other = (pos.0 + 1, pos.1);
        let hother = map.height(other);
        if hother <= height + 1 {
            output.push(other);
        }
    }
    output
}

fn solve(start: (usize, usize), input: &Input) -> usize {
    let mut paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut investigate = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let end = input.end_pos();
    investigate.insert(start);
    paths.insert(start, vec![start]);
    loop {
        let mut investigate_next: HashSet<(usize, usize)> = HashSet::new();

        // Iterate over points to investigate
        for trial in &investigate {
            let neighbors = neighbors(*trial, input);

            // Iterate over neighbors of trial point
            let curr_path = paths.get(trial).unwrap().clone();
            for n in neighbors {
                if let Some(p) = paths.get_mut(&n) {
                    // If there exists a path to n already, see if this one is
                    // shorter, insert if it is.
                    if p.len() > curr_path.len() + 1 {
                        let mut tpath = curr_path.clone();
                        tpath.push(n);
                        *p = tpath;
                    }
                } else {
                    // If there is no path to n already, use this one
                    let mut tpath = curr_path.clone();
                    tpath.push(n);
                    paths.insert(n, tpath);
                }

                // Visit n if we haven't already
                if !visited.contains(&n) {
                    investigate_next.insert(n);
                }
            }

            // Put trial in visited if it isn't already there
            visited.insert(*trial);
        }

        if investigate_next.is_empty() {
            break;
        } else {
            investigate = investigate_next.into_iter().collect();
        }
    }

    if paths.contains_key(&end) {
        paths.get(&end).unwrap().len() - 1
    } else {
        999999999
    }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    let start = input.start_pos();
    solve(start, input)
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    let mut starts: Vec<(usize, usize)> = vec![];
    for (r, row) in input.map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if let Tile::Value(0) = col {
                starts.push((c, r));
            }
        }
    }

    let mut lengths = vec![];
    for start in starts {
        lengths.push(solve(start, input));
    }
    lengths.sort();
    lengths[0]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/12.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/12.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 29);
    }
}
