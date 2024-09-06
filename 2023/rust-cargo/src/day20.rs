use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::{DiGraph, NodeIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Low,
    High,
}

impl Pulse {
    pub fn high(value: bool) -> Self {
        if value {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum Module {
    #[default]
    Sink,
    Broadcast,
    FlipFlop(bool),
    Conjuntion(HashMap<NodeIndex, Pulse>),
}

impl Module {
    pub fn process(&mut self, from: NodeIndex, pulse: Pulse) -> Option<Pulse> {
        match (self, pulse) {
            (Module::Broadcast, pulse) => Some(pulse),
            (Module::FlipFlop(_), Pulse::High) => None,
            (Module::FlipFlop(state), Pulse::Low) => {
                *state = !*state;
                Some(Pulse::high(*state))
            }
            (Module::Conjuntion(memory), pulse) => {
                *memory.get_mut(&from).unwrap() = pulse;
                Some(Pulse::high(!memory.values().all(|&p| p == Pulse::High)))
            }
            (Module::Sink, _) => None,
        }
    }
}

const PRESSES_COUNT: usize = 1000;
const BROADCAST_LABEL: &'static str = "broadcaster";
const BROADCAST_INDEX: u32 = 0;
const RX_LABEL: &'static str = "rx";
const RX_INDEX: u32 = 1;

#[aoc_generator(day20)]
pub fn generate(src: &str) -> DiGraph<Module, ()> {
    let expected_node_count = src.lines().count() + 1;
    let expected_edge_count = expected_node_count * (expected_node_count - 1);
    let mut node_indexes = HashMap::with_capacity(expected_node_count);
    let mut graph = DiGraph::with_capacity(expected_node_count, expected_edge_count);
    node_indexes.insert(BROADCAST_LABEL, graph.add_node(Module::default()));
    node_indexes.insert(RX_LABEL, graph.add_node(Module::default()));
    for line in src.lines() {
        let mut iter = line.split(" -> ");
        let src = iter.next().unwrap();
        let module = match &src[..1] {
            "%" => Module::FlipFlop(false),
            "&" => Module::Conjuntion(HashMap::with_capacity(expected_node_count - 1)),
            "b" => Module::Broadcast,
            _ => panic!(),
        };
        let label = src.trim_start_matches(&['%', '&']);
        if let Some(&src_index) = node_indexes.get(&label) {
            *graph.node_weight_mut(src_index).unwrap() = module;
        } else {
            node_indexes.insert(label, graph.add_node(module));
        }
        let dests = iter.next().unwrap().split(", ");
        for dest in dests {
            node_indexes
                .entry(&dest)
                .or_insert_with(|| graph.add_node(Module::default()));
        }
    }
    for line in src.lines() {
        let mut iter = line.split(" -> ");
        let src_index = match iter.next().unwrap() {
            BROADCAST_LABEL => BROADCAST_INDEX.into(),
            label => node_indexes[&&label[1..]],
        };
        let dest_indexes = iter.next().unwrap().split(", ").map(|l| node_indexes[&l]);
        for dest_index in dest_indexes {
            graph.add_edge(src_index, dest_index, ());
            if let Module::Conjuntion(memory) = graph.node_weight_mut(dest_index).unwrap() {
                memory.insert(src_index, Pulse::Low);
            }
        }
    }
    graph
}

#[aoc(day20, part1)]
pub fn part1_naive(input: &DiGraph<Module, ()>) -> usize {
    let broadcast_index = BROADCAST_INDEX.into();
    let mut graph = input.clone();
    let mut total_low = PRESSES_COUNT;
    let mut total_high = 0;
    let mut pulse_queue = VecDeque::new();
    let mut queue_pulse =
        |pulse: Pulse, from: NodeIndex, to: NodeIndex, queue: &mut VecDeque<_>| {
            match pulse {
                Pulse::Low => total_low += 1,
                Pulse::High => total_high += 1,
            };
            queue.push_back((pulse, from, to));
        };
    for _ in 0..PRESSES_COUNT {
        for neighbor in graph.neighbors(broadcast_index) {
            queue_pulse(Pulse::Low, broadcast_index, neighbor, &mut pulse_queue);
        }
        while let Some((pulse, from, to)) = pulse_queue.pop_front() {
            if let Some(new_pulse) = graph.node_weight_mut(to).unwrap().process(from, pulse) {
                for neighbor in graph.neighbors(to) {
                    queue_pulse(new_pulse, to, neighbor, &mut pulse_queue);
                }
            }
        }
    }
    total_low * total_high
}

#[aoc(day20, part2)]
pub fn part2_naive(input: &DiGraph<Module, ()>) -> usize {
    let broadcast = BROADCAST_INDEX.into();
    let rx = RX_INDEX.into();
    let mut graph = input.clone();
    let mut pulse_queue = VecDeque::new();
    let mut i = 0;
    loop {
        i += 1;
        for neighbor in graph.neighbors(broadcast) {
            pulse_queue.push_back((Pulse::Low, broadcast, neighbor));
        }
        while let Some((pulse, from, to)) = pulse_queue.pop_front() {
            if pulse == Pulse::Low && to == rx {
                return i;
            }
            if let Some(new_pulse) = graph.node_weight_mut(to).unwrap().process(from, pulse) {
                for neighbor in graph.neighbors(to.into()) {
                    pulse_queue.push_back((new_pulse, to, neighbor));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &'static str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    fn sample() -> DiGraph<Module, ()> {
        generate(SAMPLE_SRC)
    }

    #[test]
    pub fn part1_naive_sample() {
        assert_eq!(part1_naive(&sample()), 32000000);
    }
}
