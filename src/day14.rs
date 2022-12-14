use scan_fmt::scan_fmt;
use std::collections::HashSet;

type Input = Vec<(usize, usize)>;

// let input_iter = input.iter();
// loop {
//     let line = input_iter.next().unwrap();
// }
//
// let comma_separated = line.split(',').collect();

#[aoc_generator(day14)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        let mut temp = vec![];
        for pair_str in line.split(" -> ") {
            let (x, y) = scan_fmt!(pair_str, "{},{}", usize, usize).unwrap();
            temp.push((x, y));
        }
        output.push(temp);
    }
    output
}

fn draw_line(p1: &(usize, usize), p2: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut output = HashSet::new();
    if p1.0 == p2.0 {
        let mut miny = p1.1;
        let mut maxy = p2.1;
        if p2.1 < miny {
            miny = p2.1;
            maxy = p1.1
        }

        for i in miny..maxy + 1 {
            output.insert((p1.0, i));
        }
    } else {
        let mut minx = p1.0;
        let mut maxx = p2.0;
        if p2.0 < minx {
            minx = p2.0;
            maxx = p1.0
        }

        for i in minx..maxx + 1 {
            output.insert((i, p1.1));
        }
    }
    output
}

fn draw(input: &[Input]) -> HashSet<(usize, usize)> {
    let mut output: HashSet<(usize, usize)> = HashSet::new();
    for line in input {
        for i in 0..line.len() - 1 {
            let line_set = draw_line(&line[i], &line[i + 1]);
            output = output.union(&line_set).cloned().collect();
        }
    }

    output
}

fn iterate_sand(
    pos: (usize, usize),
    sand: &HashSet<(usize, usize)>,
    rock: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    if !rock.contains(&(pos.0, pos.1 + 1)) && !sand.contains(&(pos.0, pos.1 + 1)) {
        Some((pos.0, pos.1 + 1))
    } else if !rock.contains(&(pos.0 - 1, pos.1 + 1)) && !sand.contains(&(pos.0 - 1, pos.1 + 1)) {
        Some((pos.0 - 1, pos.1 + 1))
    } else if !rock.contains(&(pos.0 + 1, pos.1 + 1)) && !sand.contains(&(pos.0 + 1, pos.1 + 1)) {
        Some((pos.0 + 1, pos.1 + 1))
    } else {
        None
    }
}

fn sim_sand(
    sand: &mut HashSet<(usize, usize)>,
    rock: &HashSet<(usize, usize)>,
    lowest: usize,
    part2: bool,
) -> Option<(usize, usize)> {
    let entry = (500, 0);
    let mut pos = entry;

    while let Some(new_pos) = iterate_sand(pos, sand, rock) {
        pos = new_pos;
        if !part2 {
            if pos.1 >= lowest {
                // Fell into the abyss
                return None;
            }
        } else if pos.1 == lowest - 1 {
            return Some(pos);
        }
    }
    Some(pos)
}

#[aoc(day14, part1)]
fn part1(input: &[Input]) -> usize {
    let rock = draw(input);
    let mut sand = HashSet::new();

    let lowest = rock.iter().map(|r| r.1).max().unwrap();

    while let Some(new_sand) = sim_sand(&mut sand, &rock, lowest, false) {
        sand.insert(new_sand);
    }
    sand.len()
}

#[aoc(day14, part2)]
fn part2(input: &[Input]) -> usize {
    let rock = draw(input);
    let mut sand = HashSet::new();

    let lowest = rock.iter().map(|r| r.1).max().unwrap() + 2;

    let mut last_sand = (500, 0);
    while let Some(new_sand) = sim_sand(&mut sand, &rock, lowest, true) {
        if new_sand == last_sand {
            break;
        } else {
            last_sand = new_sand;
        }
        sand.insert(new_sand);
    }
    sand.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/14.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/14.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 93);
    }
}
