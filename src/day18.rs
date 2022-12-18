use rayon::prelude::*;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

type Input = (i64, i64, i64);

#[aoc_generator(day18)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        let pt = scan_fmt!(line, "{},{},{}", i64, i64, i64).unwrap();
        output.push(pt);
    }
    output
}

fn get_neighbors(cube: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let offsets = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut neighbors = vec![];
    for offset in offsets {
        neighbors.push((cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2));
    }
    neighbors
}

fn get_neighbors_on_graph(cube: (i64, i64, i64), graph: &HashSet<Input>) -> Vec<(i64, i64, i64)> {
    let offsets = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut neighbors = vec![];
    for offset in offsets {
        let candidate = (cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2);
        if graph.contains(&candidate) {
            neighbors.push(candidate);
        }
    }
    neighbors
}

#[aoc(day18, part1)]
fn part1(input: &[Input]) -> usize {
    let mut nsides = input.len() * 6;
    let cubes: HashSet<Input> = input.iter().copied().collect();
    for x in &cubes {
        let n = get_neighbors(*x);
        for nn in n {
            if cubes.contains(&nn) {
                nsides -= 1;
            }
        }
    }
    nsides
}

fn is_connected(cube1: Input, cube2: Input, graph: &HashSet<Input>) -> bool {
    let mut c1n: HashSet<Input> = HashSet::new();

    let mut investigate = HashSet::new();
    investigate.insert(cube1);
    c1n.insert(cube1);
    loop {
        let mut next_investigate = HashSet::new();
        for c in investigate {
            let n = get_neighbors_on_graph(c, graph);
            for nn in n {
                if nn == cube2 {
                    return true;
                }
                if !c1n.contains(&nn) {
                    c1n.insert(nn);
                    next_investigate.insert(nn);
                }
            }
        }

        if next_investigate.is_empty() {
            break;
        }
        investigate = next_investigate;
    }
    c1n.contains(&cube2)
}

fn _part2(input: &[Input], bound: i64) -> usize {
    let mut nsides = input.len() * 6;
    let cubes: HashSet<Input> = input.iter().copied().collect();

    // Bounding cube
    let mut big_cube: HashSet<Input> = HashSet::new();
    for x in -1..bound {
        for y in -1..bound {
            for z in -1..bound {
                big_cube.insert((x, y, z));
            }
        }
    }

    // Bounding cube less input space
    let big_cube: HashSet<Input> = big_cube.difference(&cubes).copied().collect();

    let ref_cube: Input = (-1, -1, -1);
    let interior: HashSet<Input> = big_cube
        .par_iter()
        .map(|cube| {
            if is_connected(*cube, ref_cube, &big_cube) {
                // nothing
                None
            } else {
                // trapped!
                Some(*cube)
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    for x in &cubes {
        let n = get_neighbors(*x);
        for nn in n {
            if cubes.contains(&nn) || interior.contains(&nn) {
                nsides -= 1;
            }
        }
    }
    nsides
}

#[aoc(day18, part2)]
fn part2(input: &[Input]) -> usize {
    _part2(input, 23)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/18.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/18.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(_part2(&input, 8), 58);
    }
}
