use aoc_helpers::graph::{shortest_path, Connected};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

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
            *int
        } else {
            panic!("word {} not found in mapping!", word);
        }
    }

    fn int_to_word(&self, int: usize) -> String {
        if let Some(word) = self.int_to_word_map.get(&int) {
            word.to_string()
        } else {
            panic!("int {} not found in mapping!", int);
        }
    }
}

fn path_to_string(path: &[usize], mapping: &UniqueMapping) -> String {
    let strvec: Vec<_> = path.iter().map(|&x| mapping.int_to_word(x)).collect();
    if strvec.is_empty() {
        String::new()
    } else {
        let mut output = strvec[0].to_string();
        for node in &strvec[1..] {
            output.push_str(" -> ");
            output.push_str(node);
        }
        output
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
        let words: Vec<&str> = line.split(' ').collect();
        let valve = words[1].to_string();
        let rate = words[4];
        let rate: Vec<_> = rate.split('=').collect();
        let rate: Vec<_> = rate[1].split(';').collect();
        let rate = rate[0].parse::<i64>().unwrap();
        let others: Vec<_> = words[9..]
            .iter()
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

#[derive(Clone, Debug)]
struct ValveIntMap(HashMap<usize, (i64, Vec<usize>, bool)>);

impl ValveIntMap {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Connected for ValveIntMap {
    type Item = usize;
    fn get_neighbors(&self, node: &Self::Item) -> Vec<Self::Item> {
        self.0.get(node).unwrap().1.clone()
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
    let time_walking = path_lengths.get(&(node, state.current_node)).unwrap();
    let this_time_left = state.time_left - time_walking - 1;
    let rate = *rates.get(&node).unwrap();
    let value = value_added(node, this_time_left, rate, &state.nodes_enabled);

    state.current_node = node;
    *state.nodes_enabled.get_mut(&state.current_node).unwrap() = true;
    state.total_value += value;
    state.time_left = this_time_left;
}

fn path_to_state(
    path: &[usize],
    rates: &HashMap<usize, i64>,
    nodes_enabled: &HashMap<usize, bool>,
    path_lengths: &HashMap<(usize, usize), i64>,
    mapping: &UniqueMapping,
) -> State {
    let mut state = State {
        current_node: mapping.word_to_int("AA"),
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
        if next_time >= 0 && next_time < state.1 {
            output.push((*node, next_time));
        }
    }
    output
}

fn create_path_lengths(
    possible_nodes: &HashSet<usize>,
    nodes: &ValveIntMap,
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
    let mut nodes = ValveIntMap::new();
    for line in input {
        nodes.0.insert(
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
    let rates: HashMap<usize, i64> = nodes.clone().0.iter().map(|(&k, v)| (k, v.0)).collect();
    let nodes_enabled: HashMap<usize, bool> =
        nodes.clone().0.iter().map(|(&k, v)| (k, v.2)).collect();
    let possible_nodes: HashSet<usize> = nodes.0.keys().cloned().collect();
    let path_lengths = create_path_lengths(&possible_nodes, &nodes);
    let good_nodes: HashSet<usize> = possible_nodes
        .into_iter()
        .filter(|n| nodes.0.get(n).unwrap().0 > 0)
        .collect();

    // For each useful end state...
    let mut fwdprop_map: HashMap<PathState, Vec<PathState>> = HashMap::new();
    for new_node in &good_nodes {
        for time_left in 0..30 {
            let state = (*new_node, time_left as i64);
            let paths_from = paths_from_state(state, &good_nodes, &path_lengths);
            fwdprop_map.insert(state, paths_from);
        }

        let state = (mapping.word_to_int("AA"), 30);
        let paths_from = paths_from_state(state, &good_nodes, &path_lengths);
        fwdprop_map.insert(state, paths_from);
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
            let last_node = path.last().unwrap();
            if let Some(states) = fwdprop_map.get(last_node) {
                for state in states {
                    if !path.iter().map(|p| p.0).contains(&state.0) {
                        let mut tpath = path.clone();
                        tpath.push(*state);
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
        let state = path_to_state(&int_path, &rates, &nodes_enabled, &path_lengths, &mapping);
        values.push((state.total_value, int_path));
    }

    values.sort_by(|a, b| a.0.cmp(&b.0));
    for v in &values {
        println!("{}, {}", v.0, path_to_string(&v.1, &mapping));
    }
    values.last().unwrap().0
}

#[derive(Clone, Copy, Debug)]
struct Agent<'a> {
    node: usize,
    goal: Option<usize>,
    time_left: i64,
    valve_map: &'a ValveIntMap,
    value: i64,
}

impl<'a> Agent<'a> {
    fn new(time_left: i64, start_node: usize, valve_map: &'a ValveIntMap) -> Self {
        Self {
            node: start_node,
            goal: None,
            time_left,
            valve_map: valve_map,
            value: 0,
        }
    }

    /// Returns true if new goal needs set, false otherwise
    fn time_step(&mut self, valves_enabled: &mut HashMap<usize, bool>) -> bool {
        if self.time_left > 0 {
            if Some(self.node) == self.goal && !valves_enabled.get(&self.node).unwrap() {
                // If we're at the goal and it's not already on, turn on valve
                // and get our value for it.
                *valves_enabled.get_mut(&self.node).unwrap() = true;
                let new_value = (self.time_left - 1) * self.valve_map.0.get(&self.node).unwrap().0;
                self.value += new_value;
                self.time_left -= 1;
                return true;
            } else if self.goal.is_some() {
                // Move towards goal if not there
                if let Some(path) = shortest_path(&self.goal.unwrap(), &self.node, self.valve_map) {
                    self.node = path[1];
                } else {
                    panic!("no path from node to goal");
                }
            }
            self.time_left -= 1;
        }
        false
    }
}

type Plan = (Vec<usize>, Vec<usize>);

fn simulate_path2(
    plan: &Plan,
    mapping: &UniqueMapping,
    valve_map: &ValveIntMap,
    valves_enabled: &HashMap<usize, bool>,
) -> i64 {
    let mut p1 = Agent::new(26, mapping.word_to_int("AA"), valve_map);
    let mut path1_iter = plan.0.iter().copied();
    p1.goal = path1_iter.next();
    p1.goal = path1_iter.next();

    let mut p2 = Agent::new(26, mapping.word_to_int("AA"), valve_map);
    let mut path2_iter = plan.1.iter().copied();
    p2.goal = path2_iter.next();
    p2.goal = path2_iter.next();

    let mut temp_valves_enabled = valves_enabled.clone();

    for t in 0..26 {
        if p1.time_step(&mut temp_valves_enabled) {
            p1.goal = path1_iter.next();
        }
        if p2.time_step(&mut temp_valves_enabled) {
            p2.goal = path2_iter.next();
        }
    }

    // Return total value
    p1.value + p2.value
}

#[aoc(day16, part2)]
fn part2(input: &[Input]) -> i64 {
    let mapping = populate_word_mapping(input);
    let mut valve_map = ValveIntMap::new();
    for line in input {
        valve_map.0.insert(
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
    let rates: HashMap<usize, i64> = valve_map.clone().0.iter().map(|(&k, v)| (k, v.0)).collect();
    let valves_enabled: HashMap<usize, bool> =
        valve_map.clone().0.iter().map(|(&k, v)| (k, v.2)).collect();
    let possible_nodes: HashSet<usize> = valve_map.0.keys().cloned().collect();
    let path_lengths = create_path_lengths(&possible_nodes, &valve_map);
    let good_nodes: HashSet<usize> = possible_nodes
        .into_iter()
        .filter(|n| valve_map.0.get(n).unwrap().0 > 0)
        .collect();

    // For one of us, choose some routes
    let mut all_paths: HashMap<Plan, i64> = HashMap::new();
    let combos_5: Vec<Vec<usize>> = good_nodes.iter().copied().combinations(5).collect();
    println!("combos_5 done");
    let c5_complement: Vec<Vec<usize>> = combos_5.iter().map(|c5| {
        good_nodes.iter().copied().filter(|x| !c5.contains(x)).collect()
    }).collect();
    println!("combos_5 complement done");

    let mut cntr = 0;
    for (set, cset) in combos_5.iter().zip(c5_complement.iter()) {
        println!("cntr: {}", cntr);
        cntr += 1;
        let mut paths1: Vec<Vec<usize>> = set.iter().copied().permutations(5).collect();
        let mut paths2: Vec<Vec<usize>> = cset.iter().copied().permutations(5).collect();

        paths1.insert(0, mapping.word_to_int("AA"));
        paths2.insert(0, mapping.word_to_int("AA"));

        for p1 in &paths1 {
            for p2 in &paths2 {
                let value = simulate_path2(&(p1.to_vec(), p2.to_vec()), &mapping, &valve_map, &valves_enabled);
                all_paths.insert((p1.clone(), p2.clone()), value);
            }
        }
    }
    // The example inputs
    /*
    let path1 = vec!["AA", "JJ", "BB", "CC"].iter().map(|x| mapping.word_to_int(x)).collect();
    let path2 = vec!["AA", "DD", "HH", "EE"].iter().map(|x| mapping.word_to_int(x)).collect();
    let plan = (path1, path2);
    let value = simulate_path2(&plan, &mapping, &valve_map, &valves_enabled);
    println!("Test value: {}", value);
    */

    let all_paths: Vec<(Plan, i64)> = all_paths.into_iter().collect();
    0
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
