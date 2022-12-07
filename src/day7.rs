use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Directory {
    dirs: Vec<String>,
    files: Vec<usize>,
    size: usize,
}

impl Directory {
    fn total_size(&self, dirmap: &HashMap<String, Directory>) -> usize {
        let mut size = 0;
        for dir in &self.dirs {
            if let Some(d) = dirmap.get(dir) {
                size += d.total_size(dirmap);
            } else {
                panic!("couldn't find {}", dir);
            }
        }
        for file in &self.files {
            size += file;
        }
        size
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[aoc_generator(day7)]
fn load_input(input: &str) -> HashMap<String, Directory> {
    let mut dirs: HashMap<String, Directory> = HashMap::new();
    let mut cwd = String::from("ROOT");
    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        if words[0] == "$" {
            // Command
            match words[1] {
                "cd" => {
                    if words[2] == "/" {
                        cwd = String::from("ROOT");
                    } else if words[2] == ".." {
                        let mut dirlist: Vec<_> = cwd.split('/').collect();
                        dirlist.pop();
                        cwd = dirlist.join("/").to_string();
                    } else {
                        cwd.push('/');
                        cwd.push_str(words[2]);
                    }
                }
                "ls" => {
                    dirs.insert(
                        cwd.clone(),
                        Directory {
                            dirs: vec![],
                            files: vec![],
                            size: 0,
                        },
                    );
                }
                _ => (),
            }
        } else {
            // Response
            match words[0] {
                "dir" => {
                    if let Some(d) = dirs.get_mut(&cwd) {
                        let mut temp = cwd.clone();
                        temp.push('/');
                        temp.push_str(words[1]);
                        d.dirs.push(temp);
                    }
                }
                _ => {
                    // File
                    if let Some(d) = dirs.get_mut(&cwd) {
                        d.files.push(words[0].parse::<usize>().unwrap());
                    }
                }
            }
        }
    }
    dirs
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, Directory>) -> usize {
    let mut _input = (*input).clone();
    let mut thesum = 0;
    for dir in _input.values_mut() {
        let size = dir.total_size(input);
        dir.set_size(size);
    }

    for dir in _input.values() {
        if dir.size <= 100000 {
            thesum += dir.size;
        }
    }
    thesum
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, Directory>) -> usize {
    let total_size = 70_000_000;
    let need = 30_000_000;

    let mut _input = (*input).clone();
    for dir in _input.values_mut() {
        let size = dir.total_size(input);
        dir.set_size(size);
    }

    let mut used = 0;
    if let Some(d) = _input.get("ROOT") {
        used = d.size;
    }
    let left_over = total_size - used;
    let to_free = need - left_over;
    let mut choices = vec![];
    for (dirname, dir) in _input.iter() {
        if dir.size >= to_free {
            choices.push((dir.size, dirname));
        }
    }
    choices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    choices[0].0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = load_input(&read_to_string("input/2022/07.txt").unwrap());
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = load_input(&read_to_string("input/2022/07.txt").unwrap());
        assert_eq!(part2(&input), 24933642);
    }
}
