/// TODO: Make part 2 faster!!!
///
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use rayon::prelude::*;

type Input = (Vec<(i64, i64)>, Vec<(i64, i64)>);

// let input_iter = input.iter();
// loop {
//     let line = input_iter.next().unwrap();
// }
//
// let comma_separated = line.split(',').collect();

#[aoc_generator(day15)]
fn load_input(input: &str) -> Input {
    let mut sensors = vec![];
    let mut beacons = vec![];
    for line in input.lines() {
        let (x1, y1, x2, y2) = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        sensors.push((x1, y1));
        beacons.push((x2, y2));
    }
    (sensors, beacons)
}

fn mdist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn empty_on_line(sensor: (i64, i64), beacon: (i64, i64), line: i64) -> Vec<(i64, i64)> {
    let mut output = vec![];
    let md = mdist(sensor, beacon);
    let yoffset = (sensor.1 - line).abs();
    let start = sensor.0 - md + yoffset;
    let end = sensor.0 + md - yoffset;
    for i in start..end {
        output.push((i, line));
    }
    output
}

fn interval_on_line(sensor: (i64, i64), mdist: i64, line: i64) -> Option<(i64, i64)> {
    let yoffset = (sensor.1 - line).abs();
    if yoffset > mdist {
        return None;
    }
    let start = sensor.0 - mdist + yoffset;
    let end = sensor.0 + mdist - yoffset;
    Some((start, end))
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i64 {
    let sensors = input.0.clone();
    let beacons = input.1.clone();
    let mut empty = HashSet::new();
    for (sensor, beacon) in sensors.iter().zip(beacons.iter()) {
        //let empty_spots = empty_on_line(*sensor, *beacon, 2000000);
        let empty_spots = empty_on_line(*sensor, *beacon, 11);
        for es in empty_spots {
            empty.insert(es);
        }
    }
    empty.len().try_into().unwrap()
}

fn interval(sensor: (i64, i64), mdist: i64, line: i64) -> Option<(i64, i64)> {
    // Get interval of coverage (xstart, xend)
    if (sensor.1 - line).abs() > mdist {
        return None;
    }
    
    let mut start = sensor.0 - mdist + (line - sensor.1).abs();
    if start < 0 {
        start = 0;
    }
    let mut end = sensor.0 + mdist - (line - sensor.1).abs();
    if end > 4000000 {
        end = 4000000;
    }
    Some((start, end))
}

fn intervals(sensor: (i64, i64), mdist: i64) -> Vec<((i64, i64), i64)> {
    // returns ((x0, x1), y)
    let mut ystart: i64 = sensor.1 - mdist;
    let yend: i64 = sensor.1 + mdist;
    let mut output = vec![];
    for y in ystart..yend + 1 {
        output.push((interval(sensor, mdist, y).unwrap(), y));
    }
    if sensor == (8, 7) {
        println!("(8, 7)");
        println!("mdist: {}", mdist);
        for o in &output {
            println!("{:?}", o);
        }
    }
    output
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Interval {
    ints: Vec<(i64, i64)>,
}

fn intersect(int: (i64, i64), other: (i64, i64)) -> Option<(i64, i64)> {
    if other.0 > int.1 || other.1 < int.0 {
        None
    } else {
        let mut min = int.0;
        if other.0 > min {
            min = other.0;
        }
        let mut max = int.1;
        if other.1 < max {
            max = other.1;
        }
        Some((min, max))
    }
}

fn union(int: (i64, i64), other: (i64, i64)) -> Vec<(i64, i64)> {
    let mut newints: Vec<(i64, i64)> = vec![];
    if other.0 > int.1 || other.1 < int.0 {
        newints.push(int);
        newints.push(other);
    } else {
        let mut min = int.0;
        if other.0 < min {
            min = other.0;
        }
        let mut max = int.1;
        if other.1 > max {
            max = other.1;
        }
        newints.push((min, max));
    }
    newints
}

fn difference(int: (i64, i64), other: (i64, i64)) -> Vec<(i64, i64)> {
    let mut newints: Vec<(i64, i64)> = vec![];
    if other.0 > int.1 || other.1 < int.0 {
        // disjoint
        // keep as is
        newints.push(int);
    } else if other.0 > int.0 && other.1 < int.1 {
        // split in 2
        newints.push((int.0, other.0 - 1));
        newints.push((other.1 + 1, int.1));
    } else {
        // carve off some
        if int.0 < other.0 {
            newints.push((int.0, other.0 - 1));
        } else if int.1 > other.1 {
            newints.push((other.1 + 1, int.1));
        }
    }
    newints
}

fn _part2(input: &Input, size: usize) -> i64 {
    let sensors = input.0.clone();
    let beacons = input.1.clone();
    let mdists: Vec<_> = sensors.iter().zip(beacons.iter()).map(|(&s, &b)| mdist(s, b)).collect();

    let mut bounds: HashSet<(i64, i64)> = HashSet::with_capacity(70_000_000);
    for y in 0..size as i64 + 1 {
        for i in 0..sensors.len() {
            if let Some(int) = interval_on_line(sensors[i], mdists[i], y as i64) {
                bounds.insert((int.0, y));
                bounds.insert((int.1, y));
            }
        }
    }

    println!("bounds.len(): {}", bounds.len());

    let mut candidates: Vec<(i64, i64)> = vec![];
    for (x, y) in &bounds {
        if bounds.contains(&(x + 2, *y)) && bounds.contains(&(x + 1, y + 1)) && bounds.contains(&(x + 1, y - 1)) {
            // candidate
            //println!("{:?}", (x, y));
            candidates.push((x + 1, *y));
        }
    }

    println!("candidates.len(): {}", candidates.len());

    for c in candidates {
        let mut flag = true;
        for i in 0..sensors.len() {
            if mdist(c, sensors[i]) <= mdists[i] {
                flag = false;
                break;
            }
        }

        if flag {
            return c.0*4000000 + c.1;
        }
    }
    unreachable!();
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> i64 {
    _part2(input, 4000000)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/15.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/15.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(_part2(&input, 20), 56000011);
    }
}
