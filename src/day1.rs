#[aoc_generator(day1)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut sums: Vec<u64> = vec![];
    let mut thissum = 0;
    for line in input.lines() {
        //println!("{}", line);
        if line.is_empty() {
            sums.push(thissum);
            thissum = 0;
        } else {
            thissum += line.parse::<u64>().unwrap();
        }
    }
    sums
}

#[aoc(day01, part1)]
pub fn part1(input: &[u64]) -> u64 {
    *input.iter().max().unwrap()
}

#[aoc(day01, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let mut temp: Vec<_> = input.to_vec();
    temp.sort();
    temp.iter().rev().take(3).sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 45000);
    }
}
