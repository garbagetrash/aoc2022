use std::collections::HashSet;

fn solve(input: &str, l: usize) -> usize {
    for (i, substr) in input.as_bytes().windows(l).enumerate() {
        if substr.iter().collect::<HashSet<&u8>>().len() == l {
            return i + l;
        }
    }
    unreachable!();
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let l = 4;
    solve(input, l)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let l = 14;
    solve(input, l)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
