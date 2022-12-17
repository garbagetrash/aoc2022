use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type Input = (String, i64, Vec<String>);

#[derive(Clone, Debug)]
struct UniqueMapping {
    word_to_int_map: HashMap<String, usize>,
    int_to_word_map: HashMap<usize, String>,
    cntr: usize,
}

impl UniqueMapping {
    fn new() -> Self {
        Self {
            word_to_int_map: HashMap::new(),
            int_to_word_map: HashMap::new(),
            cntr: 0,
        }
    }

    fn push(&mut self, word: &str) -> usize {
        let int = self.cntr;
        self.cntr += 1;
        self.word_to_int_map.insert(word.to_string(), int);
        self.int_to_word_map.insert(int, word.to_string());
        int
    }

    fn word_to_int(&self, word: &str) -> usize {
        if let Some(int) = self.word_to_int_map.get(word) {
            return *int;
        } else {
            panic!("word {} not found in mapping!", word);
        }
    }

    fn int_to_word(&self, int: usize) -> String {
        if let Some(word) = self.int_to_word_map.get(&int) {
            return word.to_string();
        } else {
            panic!("int {} not found in mapping!", int);
        }
    }
}

fn populate_word_mapping(lines: &[Input]) -> UniqueMapping {
    let mut mapping = UniqueMapping::new();
    for (node, _, _) in lines {
        mapping.push(node);
    }
    mapping
}

#[aoc_generator(day16)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let valve = words[1].to_string();
        let rate = words[4];
        let rate: Vec<_> = rate.split('=').collect();
        let rate: Vec<_> = rate[1].split(';').collect();
        let rate = rate[0].parse::<i64>().unwrap();
        let others: Vec<_> = words[9..]
            .into_iter()
            .map(|s| {
                let temp: Vec<_> = s.split(',').collect();
                temp[0].to_string()
            })
            .collect();

        let pline = (valve, rate, others);
        output.push(pline);
    }
    output
}

trait Connected {
    type Item;
    fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item>;
}

fn shortest_path<T, U>(start: &T, end: &T, graph: &U) -> Option<Vec<T>>
where
    T: Clone + Eq + Hash,
    U: Connected<Item = T>,
{
    let mut paths: HashMap<T, Vec<T>> = HashMap::new();
    let mut investigate: HashSet<T> = HashSet::new();
    let mut visited: HashSet<T> = HashSet::new();

    investigate.insert(end.clone());
    paths.insert(end.clone(), vec![end.clone()]);
    loop {
        let mut investigate_next: HashSet<T> = HashSet::new();

        // Iterate over nodes to investigate
        for trial in &investigate {
            let neighbors = graph.get_neighbors(&trial);

            // Iterate over neighbors of trial node
            let curr_path = paths.get(trial).unwrap().clone();
            for n in neighbors {
                if let Some(p) = paths.get_mut(&n) {
                    // If there exists a path to n already, see if this one is
                    // shorter, insert if it is.
                    if p.len() > curr_path.len() + 1 {
                        let mut tpath = curr_path.clone();
                        tpath.push(n.clone());
                        *p = tpath;
                    }
                } else {
                    // If there is no path to n already, use this one
                    let mut tpath = curr_path.clone();
                    tpath.push(n.clone());
                    paths.insert(n.clone(), tpath);
                }

                // Visit n if we haven't already
                if !visited.contains(&n) {
                    investigate_next.insert(n);
                }
            }

            // Put trial in visited if it isn't already there
            visited.insert(trial.clone());
        }

        if investigate_next.is_empty() {
            break;
        } else {
            investigate = investigate_next;
        }
    }

    paths.get(&start).cloned()
}

impl Connected for HashMap<String, (i64, Vec<String>, bool)> {
    type Item = String;
    fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item> {
        self.get(node).unwrap().1.clone()
    }
}

impl Connected for HashMap<usize, (i64, Vec<usize>, bool)> {
    type Item = usize;
    fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item> {
        self.get(node).unwrap().1.clone()
    }
}

fn value_added(valve: usize, time: i64, rate: i64, nodes_enabled: &HashMap<usize, bool>) -> i64 {
    let enabled = nodes_enabled.get(&valve).unwrap();
    if !enabled {
        rate * time
    } else {
        0
    }
}

#[derive(Clone, Debug)]
struct State {
    current_node: usize,
    time_left: i64,
    total_value: i64,
    nodes_enabled: HashMap<usize, bool>,
}

/// Base step in our graph traversal
fn goto_and_enable_node(
    node: usize,
    state: &mut State,
    rates: &HashMap<usize, i64>,
    path_lengths: &HashMap<(usize, usize), i64>,
) {
    let time_walking = path_lengths
        .get(&(node.clone(), state.current_node.clone()))
        .unwrap();
    let this_time_left = state.time_left - time_walking - 1;
    let rate = *rates.get(&node).unwrap();
    let value = value_added(node, this_time_left, rate, &state.nodes_enabled);

    state.current_node = node.clone();
    *state.nodes_enabled.get_mut(&state.current_node).unwrap() = true;
    state.total_value += value;
    state.time_left = this_time_left;
}

fn path_to_state(
    path: &[usize],
    rates: &HashMap<usize, i64>,
    nodes_enabled: &HashMap<usize, bool>,
    path_lengths: &HashMap<(usize, usize), i64>,
) -> State {
    let mut state = State {
        current_node: 0, // TODO: This needs to map to "AA"
        time_left: 30,
        total_value: 0,
        nodes_enabled: nodes_enabled.clone(),
    };
    for n in path {
        goto_and_enable_node(*n, &mut state, rates, path_lengths);
    }
    state
}

type PathState = (usize, i64);

fn paths_from_state(
    state: PathState,
    nodes: &HashSet<usize>,
    path_lengths: &HashMap<(usize, usize), i64>,
) -> Vec<PathState> {
    let mut output = vec![];
    for node in nodes {
        let length = path_lengths.get(&(*node, state.0)).unwrap();
        let next_time = state.1 - length - 1;
        if next_time >= 0 && next_time <= 30 {
            output.push((*node, next_time));
        }
    }
    output
}

fn create_path_lengths(
    possible_nodes: &HashSet<usize>,
    nodes: &HashMap<usize, (i64, Vec<usize>, bool)>,
) -> HashMap<(usize, usize), i64> {
    let mut path_lengths = HashMap::new();
    for node1 in possible_nodes {
        for node2 in possible_nodes {
            if let Some(path) = shortest_path(node1, node2, nodes) {
                path_lengths.insert((*node1, *node2), path.len() as i64 - 1);
            }
        }
    }
    path_lengths
}

#[aoc(day16, part1)]
fn part1(input: &[Input]) -> i64 {
    let mapping = populate_word_mapping(input);
    let mut nodes = HashMap::new();
    for line in input {
        nodes.insert(
            mapping.word_to_int(&line.0),
            (
                line.1,
                line.2
                    .iter()
                    .map(|k| mapping.word_to_int(k))
                    .collect::<Vec<_>>(),
                false,
            ),
        );
    }
    let mut rates: HashMap<usize, i64> = nodes.clone().iter().map(|(&k, v)| (k, v.0)).collect();
    let nodes_enabled: HashMap<usize, bool> =
        nodes.clone().iter().map(|(&k, v)| (k, v.2)).collect();
    let possible_nodes: HashSet<usize> = nodes.keys().cloned().collect();
    let path_lengths = create_path_lengths(&possible_nodes, &nodes);
    let good_nodes: HashSet<usize> = possible_nodes
        .clone()
        .into_iter()
        .filter(|n| nodes.get(n).unwrap().0 > 0)
        .collect();

    // For each useful end state...
    let mut fwdprop_map: HashMap<PathState, Vec<PathState>> = HashMap::new();
    for new_node in &good_nodes {
        for time_left in 0..31 {
            let end_state = (*new_node, time_left as i64);
            let paths_from = paths_from_state(end_state, &good_nodes, &path_lengths);
            fwdprop_map.insert(end_state, paths_from);
        }

        for time_left in 0..31 {
            let end_state = (mapping.word_to_int("AA"), time_left as i64);
            let paths_from = paths_from_state(end_state, &good_nodes, &path_lengths);
            fwdprop_map.insert(end_state, paths_from);
        }
    }

    let mut valid_paths = HashSet::<Vec<PathState>>::new();
    let start = (mapping.word_to_int("AA"), 30);
    valid_paths.insert(vec![start]);

    let mut cntr = 0;
    loop {
        println!("iter {}", cntr);
        cntr += 1;
        let mut new_paths = HashSet::new();
        for path in &valid_paths {
            //println!("propagating path: {:?}", path);
            let last_node = path.last().unwrap();
            //println!("last_node: {:?}", last_node);
            if let Some(states) = fwdprop_map.get(&last_node) {
                for state in states {
                    let int_path: Vec<usize> = path.iter().map(|p| p.0).collect();
                    if !int_path.contains(&state.0) {
                        let mut tpath = path.clone();
                        tpath.push(state.clone());
                        new_paths.insert(tpath);
                    }
                }
            }
        }

        if new_paths.is_subset(&valid_paths) {
            break;
        } else {
            for path in new_paths {
                valid_paths.insert(path);
            }
        }
    }

    println!("valid_paths.len(): {}", valid_paths.len());

    let mut values = vec![];
    for path in valid_paths {
        let int_path: Vec<usize> = path.iter().skip(1).map(|p| p.0).collect();
        let state = path_to_state(&int_path, &rates, &nodes_enabled, &path_lengths);
        values.push((state.total_value, int_path));
    }

    values.sort_by(|a, b| a.0.cmp(&b.0));
    for v in &values {
        println!("{}, {:?}", v.0, v.1);
    }
    values.last().unwrap().0
}

type PathState2 = (usize, i64, usize, i64);

fn paths_from_state2(
    state: PathState2,
    nodes: &HashSet<usize>,
    path_lengths: &HashMap<(usize, usize), i64>,
) -> Vec<PathState2> {
    let mut output = vec![];
    for node1 in nodes {
        for node2 in nodes {
            let length1 = path_lengths.get(&(*node1, state.0)).unwrap();
            let length2 = path_lengths.get(&(*node2, state.2)).unwrap();
            let time1 = state.1 - length1 - 1;
            let time2 = state.3 - length2 - 1;
            if time1 >= 0 && time1 <= 30 && time2 >= 0 && time2 <= 30 {
                output.push((*node1, time1, *node2, time2));
            }
        }
    }
    output
}

#[aoc(day16, part2)]
fn part2(input: &[Input]) -> i64 {
    let mut nodes = HashMap::new();
    let mapping = populate_word_mapping(input);
    for line in input {
        nodes.insert(
            mapping.word_to_int(&line.0),
            (
                line.1,
                line.2
                    .iter()
                    .map(|k| mapping.word_to_int(k))
                    .collect::<Vec<_>>(),
                false,
            ),
        );
    }
    let rates: HashMap<usize, i64> = nodes.clone().iter().map(|(&k, v)| (k, v.0)).collect();
    let nodes_enabled: HashMap<usize, bool> =
        nodes.clone().iter().map(|(&k, v)| (k, v.2)).collect();
    let possible_nodes: HashSet<usize> = nodes.keys().copied().collect();
    let path_lengths = create_path_lengths(&possible_nodes, &nodes);
    let good_nodes: HashSet<usize> = possible_nodes
        .clone()
        .into_iter()
        .filter(|n| nodes.get(n).unwrap().0 > 0)
        .collect();

    // For each useful end state...
    let mut fwdprop_map: HashMap<PathState2, Vec<PathState2>> = HashMap::new();
    for node1 in &good_nodes {
        for node2 in &good_nodes {
            for t1 in 0..27 {
                for t2 in 0..27 {
                    let end_state = (*node1, t1 as i64, *node2, t2 as i64);
                    let paths_from = paths_from_state2(end_state, &good_nodes, &path_lengths);
                    fwdprop_map.insert(end_state, paths_from);
                }
            }

            for t1 in 0..27 {
                for t2 in 0..27 {
                    let end_state = (
                        mapping.word_to_int("AA"),
                        t1 as i64,
                        mapping.word_to_int("AA"),
                        t2 as i64,
                    );
                    let paths_from = paths_from_state2(end_state, &good_nodes, &path_lengths);
                    fwdprop_map.insert(end_state, paths_from);
                }
            }
        }
    }

    let mut valid_paths = HashSet::<Vec<PathState2>>::new();
    let start = (mapping.word_to_int("AA"), 26, mapping.word_to_int("AA"), 26);
    valid_paths.insert(vec![start]);

    let mut cntr = 0;
    loop {
        println!("iter {}", cntr);
        cntr += 1;
        let mut new_paths = HashSet::new();
        for path in &valid_paths {
            let last_node = path.last().unwrap();
            if let Some(states) = fwdprop_map.get(last_node) {
                for state in states {
                    let int_path: Vec<usize> = path.iter().map(|p| p.0).collect();
                    if !int_path.contains(&state.0) {
                        let mut tpath = path.clone();
                        tpath.push(state.clone());
                        new_paths.insert(tpath);
                    }
                }
            }
        }

        if new_paths.is_subset(&valid_paths) {
            break;
        } else {
            for path in new_paths {
                valid_paths.insert(path);
            }
        }
    }

    println!("valid_paths.len(): {}", valid_paths.len());

    let mut values = vec![];
    for path in valid_paths {
        let int_path1: Vec<usize> = path.iter().skip(1).map(|p| p.0).collect();
        let int_path2: Vec<usize> = path.iter().skip(1).map(|p| p.2).collect();
        let state1 = path_to_state(&int_path1, &rates, &nodes_enabled, &path_lengths);
        let state2 = path_to_state(&int_path2, &rates, &nodes_enabled, &path_lengths);
        values.push(state1.total_value + state2.total_value);
    }

    values.sort();
    *values.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/16.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 1651);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/16.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1707);
    }
}
