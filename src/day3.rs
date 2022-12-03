use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn load_input(input: &str) -> String {
    String::from(input)
}

fn item_value(item: char) -> u32 {
    let ascii_value = item as u32;
    if ascii_value < 91 {
        // uppercase
        ascii_value - 65 + 27
    } else {
        // lowercase
        ascii_value - 97 + 1
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mut thesum = 0;
    for line in input.lines() {
        let (cp1, cp2) = line.split_at(line.len() / 2);
        let bag1: HashSet<char> = cp1.chars().collect();
        let bag2: HashSet<char> = cp2.chars().collect();
        for item in bag1.intersection(&bag2) {
            thesum += item_value(*item);
        }
    }
    thesum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut thesum = 0;
    for lines in input.lines().collect::<Vec<_>>().chunks(3) {
        let bag1: HashSet<char> = lines[0].chars().collect();
        let bag2: HashSet<char> = lines[1].chars().collect();
        let bag3: HashSet<char> = lines[2].chars().collect();
        let items: HashSet<_> = bag1.intersection(&bag2).copied().collect();
        let items: Vec<_> = items.intersection(&bag3).collect();
        thesum += item_value(*items[0]);
    }
    thesum
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/03a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/03a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 70);
    }
}
