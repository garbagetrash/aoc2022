use std::collections::HashMap;
use std::io::stdin;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Player {
    dir: Direction,
    pos: (usize, usize),
}

impl Player {
    fn new(map: &HashMap<(usize, usize), Tile>) -> Self {
        for col in 1..205 {
            if let Some(_) = map.get(&(1, col)) {
                return Self {
                    dir: Direction::East,
                    pos: (1, col),
                };
            }
        }
        unreachable!();
    }

    /// This method assumes you've already found a valid next position to try,
    /// and will crash if you did not.
    fn try_move(&mut self, next_pos: (usize, usize), map: &HashMap<(usize, usize), Tile>) -> bool {
        match map.get(&next_pos).unwrap() {
            Tile::Floor => {
                // We can move there
                self.pos = next_pos;
                true
            }
            Tile::Wall => {
                // We cannot move there, so just bail
                false
            }
        }
    }

    fn move_forward(&mut self, map: &HashMap<(usize, usize), Tile>) {
        let next_pos = match self.dir {
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
        };

        if let Some(_) = map.get(&next_pos) {
            // Tile is in the map
            self.try_move(next_pos, map);
        } else {
            // Tile is not in the map, meaning wrap
            match self.dir {
                Direction::North => {
                    // Find a valid row assuming max is something less than 205
                    for i in 0..205 {
                        let row = 205 - i;
                        if let Some(_) = map.get(&(row, self.pos.1)) {
                            // Found the wrap around
                            self.try_move((row, self.pos.1), map);
                            break;
                        }
                    }
                }
                Direction::East => {
                    // Find a valid col assuming starting at 1
                    for col in 1..205 {
                        if let Some(_) = map.get(&(self.pos.0, col)) {
                            // Found the wrap around
                            self.try_move((self.pos.0, col), map);
                            break;
                        }
                    }
                }
                Direction::South => {
                    // Find a valid row assuming max is something less than 205
                    for row in 1..205 {
                        if let Some(_) = map.get(&(row, self.pos.1)) {
                            // Found the wrap around
                            self.try_move((row, self.pos.1), map);
                            break;
                        }
                    }
                }
                Direction::West => {
                    // Find a valid col assuming max is something less than 205
                    for i in 0..205 {
                        let col = 205 - i;
                        if let Some(_) = map.get(&(self.pos.0, col)) {
                            // Found the wrap around
                            self.try_move((self.pos.0, col), map);
                            break;
                        }
                    }
                }
            }
        }
    }

    /// We're going to hardcode this because yeah.
    ///
    fn move_forward2(&mut self, map: &HashMap<(usize, usize), Tile>) {
        // faces are (0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)
        // if hface = 1 then it just rolls up. hface = 0 is left side, hface = 1 is right
        let vface = (self.pos.0 - 1) / 50; // 0, 1, 2, or 3
        let hface = (self.pos.1 - 1) / 50; // 0, 1, or 2
        let next_pos = match self.dir {
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
        };

        if let Some(_) = map.get(&next_pos) {
            // Tile is in the map
            self.try_move(next_pos, map);
        } else {
            // Tile is not in the map, meaning wrap
            match self.dir {
                Direction::North => {
                    let (try_pos, dir): ((usize, usize), Direction) = match hface {
                        0 => ((50 + self.pos.1, 51), Direction::East),
                        1 => ((150 + (self.pos.1 - 50), 1), Direction::East),
                        2 => ((200, self.pos.1 - 100), Direction::North),
                        _ => unreachable!(),
                    };
                    if self.try_move(try_pos, map) {
                        // Change direction accordingly
                        self.dir = dir;
                    }
                }
                Direction::East => {
                    let (try_pos, dir): ((usize, usize), Direction) = match vface {
                        0 => ((151 - self.pos.0, 100), Direction::West),
                        1 => ((50, 100 + (self.pos.0 - 50)), Direction::North),
                        2 => ((51 - (self.pos.0 - 100), 150), Direction::West),
                        3 => ((150, 50 + (self.pos.0 - 150)), Direction::North),
                        _ => unreachable!(),
                    };
                    if self.try_move(try_pos, map) {
                        // Change direction accordingly
                        self.dir = dir;
                    }
                }
                Direction::South => {
                    let (try_pos, dir): ((usize, usize), Direction) = match hface {
                        0 => ((1, self.pos.1 + 100), Direction::South),
                        1 => ((150 + (self.pos.1 - 50), 50), Direction::West),
                        2 => ((50 + (self.pos.1 - 100), 100), Direction::West),
                        _ => unreachable!(),
                    };
                    if self.try_move(try_pos, map) {
                        // Change direction accordingly
                        self.dir = dir;
                    }
                }
                Direction::West => {
                    let (try_pos, dir): ((usize, usize), Direction) = match vface {
                        0 => ((151 - self.pos.0, 1), Direction::East),
                        1 => ((101, self.pos.0 - 50), Direction::South),
                        2 => ((51 - (self.pos.0 - 100), 51), Direction::East),
                        3 => ((1, 50 + self.pos.0 - 150), Direction::South),
                        _ => unreachable!(),
                    };
                    if self.try_move(try_pos, map) {
                        // Change direction accordingly
                        self.dir = dir;
                    }
                }
            }
        }
    }

    fn steer(&mut self, steer: char) {
        match steer {
            'L' => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::East => self.dir = Direction::North,
                Direction::South => self.dir = Direction::East,
                Direction::West => self.dir = Direction::South,
            },
            'R' => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::North,
            },
            _ => unreachable!(),
        }
    }
}

type Input = (String, HashMap<(usize, usize), Tile>);

#[aoc_generator(day22)]
fn load_input(input: &str) -> Input {
    let mut output = HashMap::new();
    let mut line_iter = input.lines().enumerate();
    loop {
        let (row, line) = line_iter.next().unwrap();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    output.insert((row + 1, col + 1), Tile::Floor);
                }
                '#' => {
                    output.insert((row + 1, col + 1), Tile::Wall);
                }
                ' ' => (),
                _ => break,
            }
        }
        if line == "" {
            break;
        }
    }
    let instructions = line_iter.next().unwrap().1.to_string();
    (instructions, output)
}

fn draw_board(player: &Player, map: &HashMap<(usize, usize), Tile>, num: &str, instructions: &str) {
    println!();
    println!("Instructions: {}", instructions);
    println!("Number: {}", num);
    println!();

    let rstart = if player.pos.0 < 19 {
        0
    } else {
        player.pos.0 - 20
    };

    let rstop = if player.pos.0 > 180 {
        205
    } else {
        player.pos.0 + 20
    };

    for row in rstart..rstop {
        for col in 1..180 {
            if (row, col) == player.pos {
                print!("P");
                continue;
            }
            let mut c = ' ';
            if let Some(tile) = map.get(&(row, col)) {
                c = match tile {
                    Tile::Floor => '.',
                    Tile::Wall => '#',
                }
            }
            print!("{}", c);
        }
        println!();
    }
}

// 106158 is too high
#[aoc(day22, part1)]
fn part1(input: &Input) -> usize {
    let (instructions, map) = input;

    let mut player = Player::new(map);

    // Execute instructions
    let mut char_iter = instructions.chars();
    let mut num = String::new();
    loop {
        //draw_board(&player, &map, &num, &instructions);
        //let mut asdf = String::new();
        //stdin().read_line(&mut asdf);

        if let Some(c) = char_iter.next() {
            if c.is_alphabetic() {
                // Make the last move, if any
                if num.len() > 0 {
                    let move_len = num.parse::<usize>().unwrap();
                    num = String::new();
                    for _ in 0..move_len {
                        player.move_forward(map);
                    }
                }

                // Steering
                player.steer(c);
            } else if c.is_alphanumeric() {
                // Gather characters into the move length string
                num.push(c);
            } else {
                // Probably a new line at the end, bail
                break;
            }
        } else {
            // iter is empty, bail
            break;
        }
    }

    // Make the last move, if any
    if num.len() > 0 {
        let move_len = num.parse::<usize>().unwrap();
        num = String::new();
        for _ in 0..move_len {
            player.move_forward(map);
        }
    }

    let facing = match player.dir {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
    };
    player.pos.0 * 1000 + player.pos.1 * 4 + facing
}

// 164086 is too high
#[aoc(day22, part2)]
fn part2(input: &Input) -> usize {
    let (instructions, map) = input;

    let mut player = Player::new(map);

    // Execute instructions
    let mut char_iter = instructions.chars();
    let mut num = String::new();
    loop {
        //draw_board(&player, &map, &num, &instructions);
        //let mut asdf = String::new();
        //stdin().read_line(&mut asdf);

        if let Some(c) = char_iter.next() {
            if c.is_alphabetic() {
                // Make the last move, if any
                if num.len() > 0 {
                    let move_len = num.parse::<usize>().unwrap();
                    num = String::new();
                    for _ in 0..move_len {
                        player.move_forward2(map);
                    }
                }

                // Steering
                player.steer(c);
            } else if c.is_alphanumeric() {
                // Gather characters into the move length string
                num.push(c);
            } else {
                // Probably a new line at the end, bail
                break;
            }
        } else {
            // iter is empty, bail
            break;
        }
    }

    // Make the last move, if any
    if num.len() > 0 {
        let move_len = num.parse::<usize>().unwrap();
        num = String::new();
        for _ in 0..move_len {
            player.move_forward2(map);
        }
    }

    let facing = match player.dir {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
    };
    player.pos.0 * 1000 + player.pos.1 * 4 + facing
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/22.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 6032);
    }

    /* lol im not going to solve this for general case foldings
    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/22.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
    */
}
