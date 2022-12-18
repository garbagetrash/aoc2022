use std::collections::HashSet;

#[allow(unused_imports)]
use std::io::stdin;

type Input = char;

#[derive(Copy, Clone, Debug)]
enum RockType {
    HLine,
    Plus,
    LFlipped,
    VLine,
    Square,
}

type Rock = (i64, i64, RockType);

// Edges relative to upper left corner of shape
const ROCK_RIGHT_EDGE: [i64; 5] = [3, 2, 2, 0, 1];
const ROCK_BOTTOM_EDGE: [i64; 5] = [0, -2, -2, -3, -1];

fn rock_types() -> Vec<Rock> {
    vec![
        (0, 2, RockType::HLine),
        (0, 2, RockType::Plus),
        (0, 2, RockType::LFlipped),
        (0, 2, RockType::VLine),
        (0, 2, RockType::Square),
    ]
}

fn rock_shapes() -> Vec<Vec<(i64, i64)>> {
    vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],             // hline `-`
        vec![(0, 1), (-1, 0), (-1, 1), (-1, 2), (-2, 1)], // plus `+`
        vec![(0, 2), (-1, 2), (-2, 0), (-2, 1), (-2, 2)], // hflipped `L`
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],          // vline `|`
        vec![(0, 0), (0, 1), (-1, 0), (-1, 1)],           // box `#`
    ]
}

#[derive(Clone, Debug)]
struct RockIter {
    idx: usize,
    rocks: Vec<Rock>,
}

impl RockIter {
    fn new() -> Self {
        Self {
            idx: 0,
            rocks: rock_types(),
        }
    }
}

impl Iterator for RockIter {
    type Item = Rock;
    fn next(&mut self) -> Option<Self::Item> {
        let output = Some(self.rocks[self.idx]);
        self.idx += 1;
        self.idx %= 5;
        output
    }
}

struct InputIter {
    idx: usize,
    input: Vec<char>,
}

impl InputIter {
    fn new(input: &[char]) -> Self {
        Self {
            idx: 0,
            input: input.to_vec(),
        }
    }
}

impl Iterator for InputIter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let output = Some(self.input[self.idx]);
        self.idx += 1;
        self.idx %= self.input.len();
        output
    }
}

#[aoc_generator(day17)]
fn load_input(input: &str) -> Vec<Input> {
    input.trim().chars().collect()
}

fn wind_move_rock(
    rock: Rock,
    wind: char,
    rock_shapes: &[Vec<(i64, i64)>],
    board: &HashSet<(i64, i64)>,
) -> Rock {
    match wind {
        '<' => {
            if rock.1 > 0 {
                // Edge not blocking movement, what about board?
                let shape = &rock_shapes[rock.2 as usize];
                let mut blocked = false;
                for pt in shape {
                    let next_pos = (pt.0 + rock.0, pt.1 + rock.1 - 1);
                    if board.contains(&next_pos) {
                        blocked = true;
                        break;
                    }
                }
                if !blocked {
                    (rock.0, rock.1 - 1, rock.2)
                } else {
                    rock
                }
            } else {
                rock
            }
        }
        '>' => {
            let right = ROCK_RIGHT_EDGE[rock.2 as usize];
            if rock.1 + right < 6 {
                // Edge not blocking movement, what about board?
                let shape = &rock_shapes[rock.2 as usize];
                let mut blocked = false;
                for pt in shape {
                    let next_pos = (pt.0 + rock.0, pt.1 + rock.1 + 1);
                    if board.contains(&next_pos) {
                        blocked = true;
                        break;
                    }
                }
                if !blocked {
                    (rock.0, rock.1 + 1, rock.2)
                } else {
                    rock
                }
            } else {
                rock
            }
        }
        _ => panic!("Invalid wind direction"),
    }
}

fn move_down_rock(
    rock: Rock,
    rock_shapes: &[Vec<(i64, i64)>],
    board: &HashSet<(i64, i64)>,
) -> (Rock, bool) {
    if rock.0 > 0 {
        // Floor not blocking movement, what about board?
        let shape = &rock_shapes[rock.2 as usize];
        let mut blocked = false;
        for pt in shape {
            let next_pos = (pt.0 + rock.0 - 1, pt.1 + rock.1);
            if board.contains(&next_pos) {
                blocked = true;
                break;
            }
        }
        if !blocked {
            ((rock.0 - 1, rock.1, rock.2), false)
        } else {
            (rock, true)
        }
    } else {
        (rock, true)
    }
}

#[allow(dead_code)]
fn draw_board(board: &HashSet<(i64, i64)>, offset: i64) {
    for h in 0..50 {
        print!("|");
        for w in 0..7 {
            if board.contains(&(offset + 50 - h, w)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

fn rock_iter(
    riter: &mut RockIter,
    input_iter: &mut InputIter,
    board: &mut HashSet<(i64, i64)>,
    rshapes: &[Vec<(i64, i64)>],
) {
    let mut next_rock = riter.next().unwrap();

    // Set the height to 3 above tallest point in board
    let highest_point = board.iter().map(|p| p.0).max().unwrap_or(0);
    next_rock.0 += highest_point + 4 - ROCK_BOTTOM_EDGE[next_rock.2 as usize];

    loop {
        let wind = input_iter.next().unwrap();
        //println!("\nwind: {}", wind);
        //println!("rock: {:?}", next_rock);

        //draw_board(&board, 0);
        //let mut asdf = String::new();
        //stdin().read_line(&mut asdf);

        let shifted_rock = wind_move_rock(next_rock, wind, rshapes, board);
        let (fallen_rock, done_falling) = move_down_rock(shifted_rock, rshapes, board);
        if done_falling {
            for point in &rshapes[next_rock.2 as usize] {
                let shifted_point = (point.0 + fallen_rock.0, point.1 + fallen_rock.1);
                board.insert(shifted_point);
            }
            break;
        } else {
            next_rock = fallen_rock;
        }
    }
}

fn solve(input: &[Input], nrocks: usize) -> i64 {
    let mut input_iter = InputIter::new(input);
    let rshapes = rock_shapes();
    let mut riter = RockIter::new();
    let mut board: HashSet<(i64, i64)> = HashSet::new();
    for x in 0..7 {
        board.insert((0, x));
    }
    for _ in 0..nrocks {
        rock_iter(&mut riter, &mut input_iter, &mut board, &rshapes);
    }

    board.iter().map(|p| p.0).max().unwrap_or(0)
}

#[aoc(day17, part1)]
fn part1(input: &[Input]) -> i64 {
    solve(input, 2022)
}

fn find_cycle_height(board: &HashSet<(i64, i64)>, offset: i64, window: i64) -> i64 {
    let height = board.iter().map(|p| p.0).max().unwrap_or(0);

    // Save the first 40 lines
    let mut pattern: HashSet<(i64, i64)> = HashSet::new();
    for i in 0..window {
        for x in 0..7 {
            if board.contains(&(i + offset, x)) {
                pattern.insert((i, x));
            }
        }
    }

    let mut i = offset + 1;
    loop {
        if i > height {
            panic!("find_cycle_height failed");
        }
        let mut chunk: HashSet<(i64, i64)> = HashSet::new();
        for j in 0..window {
            for x in 0..7 {
                if board.contains(&(i + j, x)) {
                    chunk.insert((j, x));
                }
            }
        }

        if pattern.is_subset(&chunk) {
            return i - offset;
        } else {
            i += 1;
        }
    }
}

/// NOTE: This works because we have 5 shapes that repeat, and a length 40
/// input that repeats, meaning the inputs just cycle every 40 blocks. IF
/// nothing slips through the gaps then it's clean.
#[aoc(day17, part2)]
fn part2(input: &[Input]) -> i64 {
    let mut input_iter = InputIter::new(input);
    let rshapes = rock_shapes();
    let mut riter = RockIter::new();
    let mut board: HashSet<(i64, i64)> = HashSet::new();
    for x in 0..7 {
        board.insert((0, x));
    }

    for _ in 0..4000 {
        rock_iter(&mut riter, &mut input_iter, &mut board, &rshapes);
    }

    let height = board.iter().map(|p| p.0).max().unwrap_or(0);

    let offset = 500;
    let window = 40;
    let repeat_height = find_cycle_height(&board, offset, window);

    // Save a pattern
    let mut pattern: HashSet<(i64, i64)> = HashSet::new();
    for i in 0..window {
        for x in 0..7 {
            if board.contains(&(height - i, x)) {
                pattern.insert((i, x));
            }
        }
    }

    let repeat_time;
    let mut cntr = 1;
    loop {
        rock_iter(&mut riter, &mut input_iter, &mut board, &rshapes);
        let height = board.iter().map(|p| p.0).max().unwrap_or(0);

        let mut chunk: HashSet<(i64, i64)> = HashSet::new();
        for i in 0..window {
            for x in 0..7 {
                if board.contains(&(height - i, x)) {
                    chunk.insert((i, x));
                }
            }
        }

        if pattern.is_subset(&chunk) {
            repeat_time = cntr;
            break;
        } else {
            cntr += 1;
        }
    }

    let next_sim: i64 = 1000000000000 % repeat_time;
    let ncycles: i64 = 1000000000000 / repeat_time;

    let mut height_so_far = repeat_height * ncycles;
    height_so_far += solve(input, next_sim as usize);
    height_so_far
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/17.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/17.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1514285714288);
    }
}
