use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

type Input = (Vec<(i64, i64)>, Vec<(i64, i64)>);

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

fn empty_on_line(sensor: (i64, i64), beacon: (i64, i64), line: i64) -> Vec<i64> {
    let mut output = vec![];
    let md = mdist(sensor, beacon);
    let yoffset = (sensor.1 - line).abs();
    let start = sensor.0 - md + yoffset;
    let end = sensor.0 + md - yoffset;
    for i in start..end {
        output.push(i);
    }
    output
}

fn _part1(input: &Input, line: i64) -> i64 {
    let sensors = input.0.clone();
    let beacons = input.1.clone();
    let empty: HashSet<i64> = sensors
        .into_iter()
        .zip(beacons.iter())
        .flat_map(|(sensor, beacon)| empty_on_line(sensor, *beacon, line))
        .collect();
    empty.len().try_into().unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i64 {
    _part1(input, 2000000)
}

/// Line format (slope, yintercept)
fn lines_from_sensor(sensor: (i64, i64), dist: i64) -> Vec<(i64, i64)> {
    vec![
        (-1, sensor.1 + dist + 1 + sensor.0), // NE
        (1, sensor.1 - dist - 1 - sensor.0),  // SE
        (-1, sensor.1 - dist - 1 + sensor.0), // SW
        (1, sensor.1 + dist + 1 - sensor.0),  // NW
    ]
}

fn line_intersection(nline: (i64, i64), pline: (i64, i64)) -> (i64, i64) {
    let x = (nline.1 - pline.1) / 2;
    let y = x + pline.1;
    (x, y)
}

fn _part2(input: &Input, size: usize) -> i64 {
    let sensors = input.0.clone();
    let beacons = input.1.clone();
    let mdists: Vec<_> = sensors
        .iter()
        .zip(beacons.iter())
        .map(|(&s, &b)| mdist(s, b))
        .collect();

    let mut all_lines: HashMap<(i64, i64), usize> = HashMap::new();

    for i in 0..sensors.len() {
        let new_lines = lines_from_sensor(sensors[i], mdists[i]);
        for line in new_lines {
            *all_lines.entry(line).or_insert(1) += 1;
        }
    }

    let nlines: Vec<_> = all_lines.keys().filter(|&k| k.0 < 0).collect();
    let plines: Vec<_> = all_lines.keys().filter(|&k| k.0 > 0).collect();

    let mut candidates = vec![];
    for nline in nlines {
        for pline in &plines {
            let cpt = line_intersection(*nline, **pline);
            if cpt.0 >= 0 && cpt.1 >= 0 && cpt.0 <= size as i64 && cpt.1 <= size as i64 {
                candidates.push(cpt);
            }
        }
    }

    for c in candidates {
        let mut flag = true;
        for i in 0..sensors.len() {
            if mdist(c, sensors[i]) <= mdists[i] {
                flag = false;
                break;
            }
        }

        if flag {
            return c.0 * 4000000 + c.1;
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
        assert_eq!(_part1(&input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/15.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(_part2(&input, 20), 56000011);
    }
}
