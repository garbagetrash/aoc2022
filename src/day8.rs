#![allow(clippy::needless_range_loop)]
#[aoc_generator(day8)]
fn load_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut temp = vec![];
        for c in line.chars() {
            temp.push(c.to_digit(10).unwrap());
        }
        output.push(temp);
    }
    output
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let nrows = input.len();
    let ncols = input[0].len();

    let mut is_visible: Vec<Vec<u32>> = vec![vec![0; ncols]; nrows];
    is_visible[0][0] = 1;
    is_visible[0][ncols - 1] = 1;
    is_visible[nrows - 1][0] = 1;
    is_visible[nrows - 1][ncols - 1] = 1;

    for x in 1..nrows - 1 {
        // look from left
        let mut row_max = input[x][0];
        is_visible[x][0] = 1;
        for y in 1..ncols - 1 {
            if input[x][y] > row_max {
                row_max = input[x][y];
                is_visible[x][y] = 1;
            }
            if row_max == 9 {
                break;
            }
        }

        // look from right
        let mut row_max = input[x][ncols - 1];
        is_visible[x][ncols - 1] = 1;
        for y in (1..ncols - 1).rev() {
            if input[x][y] > row_max {
                row_max = input[x][y];
                is_visible[x][y] = 1;
            }
            if row_max == 9 {
                break;
            }
        }
    }

    for y in 1..ncols - 1 {
        // look from top
        let mut col_max = input[0][y];
        is_visible[0][y] = 1;
        for x in 1..nrows - 1 {
            if input[x][y] > col_max {
                col_max = input[x][y];
                is_visible[x][y] = 1;
            }
            if col_max == 9 {
                break;
            }
        }

        // look from bottom
        let mut col_max = input[nrows - 1][y];
        is_visible[nrows - 1][y] = 1;
        for x in (1..nrows - 1).rev() {
            if input[x][y] > col_max {
                col_max = input[x][y];
                is_visible[x][y] = 1;
            }
            if col_max == 9 {
                break;
            }
        }
    }
    is_visible.iter().flatten().sum::<u32>()
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let nrows = input.len();
    let ncols = input[0].len();

    let mut score: Vec<Vec<u32>> = vec![vec![1; ncols]; nrows];

    for x in 1..nrows - 1 {
        for y in 1..ncols - 1 {
            let height = input[x][y];

            // look left
            let mut left_score = 0;
            for yy in (0..y).rev() {
                left_score += 1;
                if input[x][yy] >= height {
                    break;
                }
            }
            // look right
            let mut right_score = 0;
            for yy in y + 1..ncols {
                right_score += 1;
                if input[x][yy] >= height {
                    break;
                }
            }
            // look up
            let mut up_score = 0;
            for xx in (0..x).rev() {
                up_score += 1;
                if input[xx][y] >= height {
                    break;
                }
            }
            // look down
            let mut down_score = 0;
            for xx in x + 1..nrows {
                down_score += 1;
                if input[xx][y] >= height {
                    break;
                }
            }
            score[x][y] = left_score * right_score * up_score * down_score;
        }
    }
    *score.iter().flatten().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = load_input(&read_to_string("input/2022/08.txt").unwrap());
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = load_input(&read_to_string("input/2022/08.txt").unwrap());
        assert_eq!(part2(&input), 8);
    }
}
