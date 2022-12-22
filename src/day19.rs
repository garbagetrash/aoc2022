use aoc_helpers::graph::Graph;
use rayon::prelude::*;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Blueprint {
    idnum: i64,
    orebot_cost: i64,        // ore
    claybot_cost: i64,       // ore
    obsbot_cost: (i64, i64), // (ore, clay)
    geobot_cost: (i64, i64), // (ore, obsidian)
}

impl Blueprint {
    fn new(input: (i64, i64, i64, i64, i64, i64, i64)) -> Self {
        Self {
            idnum: input.0,
            orebot_cost: input.1,
            claybot_cost: input.2,
            obsbot_cost: (input.3, input.4),
            geobot_cost: (input.5, input.6),
        }
    }
}

type Input = Blueprint;

#[aoc_generator(day19)]
fn load_input(input: &str) -> Vec<Input> {
    let mut output = vec![];
    for line in input.lines() {
        // (bnum, orebot, claybot, obsbot_ore, obsbot_clay, geobot_ore, geobot_obs)
        let temp = scan_fmt!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", i64, i64, i64, i64, i64, i64, i64).unwrap();
        output.push(Blueprint::new(temp));
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    ore_bots: i64,
    clay_bots: i64,
    obs_bots: i64,
    geo_bots: i64,
    ore: i64,
    clay: i64,
    obs: i64,
    geo: i64,
}

impl State {
    fn new() -> Self {
        Self {
            ore_bots: 1,
            clay_bots: 0,
            obs_bots: 0,
            geo_bots: 0,
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
        }
    }
}

// 0 - orebot
// 1 - claybot
// 2 - obsbot
// 3 - geobot
fn time_step(new_bot: Option<i64>, state: &mut State, blueprint: Blueprint) {
    // Spend resources on new bots
    match new_bot {
        Some(0) => {
            state.ore -= blueprint.orebot_cost;
        }
        Some(1) => {
            state.ore -= blueprint.claybot_cost;
        }
        Some(2) => {
            state.ore -= blueprint.obsbot_cost.0;
            state.clay -= blueprint.obsbot_cost.1;
        }
        Some(3) => {
            state.ore -= blueprint.geobot_cost.0;
            state.obs -= blueprint.geobot_cost.1;
        }
        _ => (),
    }

    // Bots collect resources
    state.ore += state.ore_bots;
    state.clay += state.clay_bots;
    state.obs += state.obs_bots;
    state.geo += state.geo_bots;

    // Bots are built
    match new_bot {
        Some(0) => state.ore_bots += 1,
        Some(1) => state.clay_bots += 1,
        Some(2) => state.obs_bots += 1,
        Some(3) => state.geo_bots += 1,
        _ => (),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DState {
    time: i64,
    money: (i64, i64, i64, i64),
    nbots: (i64, i64, i64, i64),
}

impl DState {
    fn new(time: i64, state: State) -> Self {
        Self {
            time,
            money: (state.ore, state.clay, state.obs, state.geo),
            nbots: (
                state.ore_bots,
                state.clay_bots,
                state.obs_bots,
                state.geo_bots,
            ),
        }
    }

    fn to_state(&self) -> State {
        let mut s = State::new();
        s.ore_bots = self.nbots.0;
        s.clay_bots = self.nbots.1;
        s.obs_bots = self.nbots.2;
        s.geo_bots = self.nbots.3;
        s.ore = self.money.0;
        s.clay = self.money.1;
        s.obs = self.money.2;
        s.geo = self.money.3;
        s
    }
}

fn metric(dstate: DState, blueprint: Blueprint) -> i64 {
   dstate.nbots.3 + 10 * dstate.money.3
}

// 851 too low
// 1032 too low
// 1114
// 1117
// 1121 too low
#[aoc(day19, part1)]
fn part1(input: &[Input]) -> usize {
    let mut answers = vec![];
    for (bnum, blueprint) in input.iter().enumerate() {
        let obs_time = blueprint.geobot_cost.1 + 1;
        let mut state = State::new();
        let mut explored_states = Vec::with_capacity(1_000_000);
        let mut dstates_to_investigate = Vec::with_capacity(1_000_000);
        dstates_to_investigate.push(DState::new(24, state));
        let mut asdf = String::new();
        let mut cntr = 0;

        let mut max_ore = vec![
            blueprint.orebot_cost,
            blueprint.claybot_cost,
            blueprint.obsbot_cost.0,
            blueprint.geobot_cost.0,
        ];
        let max_ore = max_ore.into_iter().max().unwrap();
        let max_clay = blueprint.obsbot_cost.1;
        let max_obs = blueprint.geobot_cost.1;
        loop {
            /*
            stdin().read_line(&mut asdf);
            println!();
            println!("# explored_states: {}", explored_states.len());
            println!("# dstates_to_investigate: {}", dstates_to_investigate.len());
            */

            let dstate = dstates_to_investigate.pop().unwrap(); // try the next highest priority state
                                                                //println!("  {:?}", dstate);

            // If time remaining, explore frontier of dstate
            let mut next_dstates = vec![];
            if dstate.time > 0 {
                let state = dstate.to_state();

                // Calculate the options from this `State`
                // Pre pruning trick: Don't build more of a bot than any other bot takes to build
                // as input.
                let mut options = vec![None];
                if state.ore >= blueprint.orebot_cost && state.ore_bots < max_ore {
                    // Orebot can be built
                    options.push(Some(0));
                }
                if state.ore >= blueprint.claybot_cost && state.clay_bots < max_clay {
                    // Claybot can be built
                    options.push(Some(1));
                }
                if state.ore >= blueprint.obsbot_cost.0
                    && state.clay >= blueprint.obsbot_cost.1
                    && state.obs_bots < max_obs
                {
                    // Obsbot can be built
                    options.push(Some(2));
                }
                if state.ore >= blueprint.geobot_cost.0 && state.obs >= blueprint.geobot_cost.1 {
                    // Geobot can be built
                    options.push(Some(3));
                }

                // Find possible next DStates
                for choice in &options {
                    let mut temp_state = state;
                    time_step(*choice, &mut temp_state, *blueprint);
                    next_dstates.push(DState::new(dstate.time - 1, temp_state));
                }
            }

            // Keep track of explored states
            explored_states.push(dstate);

            // Some heuristics
            // If less than 1 obsbot with 5 or less steps left, probably not it.
            dstates_to_investigate.append(&mut next_dstates);

            // Count terminal nodes
            if cntr % 100_000 == 0 {
                dstates_to_investigate
                    .sort_by(|a, b| metric(*a, *blueprint).cmp(&metric(*b, *blueprint)));
                let n_terminal_nodes = explored_states.iter().filter(|x| x.time == 0).count();
                println!("# Terminal States: {}", n_terminal_nodes);
                println!("# Explored States: {}", explored_states.len());
                if n_terminal_nodes > 1_000_000 {
                    break;
                }
            }
            cntr += 1;
        }

        let mut terminal_states: Vec<DState> = explored_states
            .into_iter()
            .filter(|x| x.time == 0)
            .collect();

        terminal_states.sort_by(|a, b| a.money.3.cmp(&b.money.3));
        let max_geodes = terminal_states.last().unwrap().money.3;
        println!("# Geodes: {}", max_geodes);
        answers.push((bnum + 1) * max_geodes as usize);
    }
    answers.iter().sum::<usize>()
}

#[aoc(day19, part2)]
fn part2(input: &[Input]) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2022/19.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2022/19.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
