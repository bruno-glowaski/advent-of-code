use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use grid::Grid;
use petgraph::visit::EdgeRef;

use crate::utils::grid::{grid_translate, parse_grid_with_u8, S4Direction, ALL_S4_DIRECTIONS};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    #[derive(Debug, PartialEq, Eq)]
    enum Tile {
        Solid,
        Empty,
        Slope(S4Direction),
    }

    impl Tile {
        fn from_u8(value: u8) -> Self {
            match value {
                b'#' => Self::Solid,
                b'.' => Self::Empty,
                b'^' => Self::Slope(S4Direction::North),
                b'>' => Self::Slope(S4Direction::East),
                b'v' => Self::Slope(S4Direction::South),
                b'<' => Self::Slope(S4Direction::West),
                c => panic!("Unexpected char: {}", c as char),
            }
        }

        fn is_empty(&self) -> bool {
            self == &Tile::Empty
        }

        fn is_solid(&self) -> bool {
            self == &Tile::Solid
        }
    }

    fn walk_to(
        direction: S4Direction,
        coords: (usize, usize),
        dimensions: &(usize, usize),
        tile_map: &Grid<Tile>,
        distances: &mut Grid<usize>,
        directions: &mut Grid<Option<S4Direction>>,
        queue: &mut VecDeque<(usize, usize)>,
    ) {
        if directions[coords]
            .map(|d| direction == d.reverse())
            .unwrap_or(false)
        {
            return;
        }
        let new_coord = match grid_translate(coords, direction.vector(1), &dimensions) {
            Some(c) => c,
            None => return,
        };
        if tile_map[new_coord].is_solid() {
            return;
        }
        let expected_distance = distances[coords] + 1;
        if !(distances[new_coord] < expected_distance) {
            return;
        }
        distances[new_coord] = expected_distance;
        directions[new_coord] = Some(direction);
        queue.push_back(new_coord);
    }

    let tile_map = parse_grid_with_u8(input, |_, c| Tile::from_u8(c));
    let dimensions = (tile_map.rows(), tile_map.rows());
    let start_point = (0, tile_map.iter_row(0).position(|b| b.is_empty()).unwrap());
    let end_point = (
        dimensions.0 - 1,
        tile_map
            .iter_row(dimensions.0 - 1)
            .position(|b| b.is_empty())
            .unwrap(),
    );
    let mut distances = Grid::new(dimensions.0, dimensions.1);
    let mut directions = Grid::new(dimensions.0, dimensions.1);
    directions[start_point] = Some(S4Direction::South);
    let mut queue = VecDeque::with_capacity(dimensions.0);
    queue.push_back(start_point);
    while let Some(coords) = queue.pop_front() {
        match tile_map[coords] {
            Tile::Solid => {}
            Tile::Empty => {
                for direction in ALL_S4_DIRECTIONS {
                    walk_to(
                        direction,
                        coords,
                        &dimensions,
                        &tile_map,
                        &mut distances,
                        &mut directions,
                        &mut queue,
                    );
                }
            }
            Tile::Slope(direction) => walk_to(
                direction,
                coords,
                &dimensions,
                &tile_map,
                &mut distances,
                &mut directions,
                &mut queue,
            ),
        }
    }
    distances[end_point]
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    use petgraph::prelude::UnGraph;

    const START_INDEX: usize = 0;
    const END_INDEX: usize = 1;

    pub fn parse_graph(input: &str) -> UnGraph<(), usize, usize> {
        let collision_map = parse_grid_with_u8(input, |_, c| c == b'#');
        let dimensions = (collision_map.rows(), collision_map.rows());
        let start_point = (0, collision_map.iter_row(0).position(|b| !b).unwrap());
        let end_point = (
            dimensions.0 - 1,
            collision_map
                .iter_row(dimensions.0 - 1)
                .position(|b| !b)
                .unwrap(),
        );
        let mut graph = UnGraph::default();
        let mut node_map = Grid::new(dimensions.0, dimensions.1);
        node_map[start_point] = Some(graph.add_node(()));
        node_map[end_point] = Some(graph.add_node(()));
        let mut visited = Grid::new(dimensions.0, dimensions.1);
        let mut stack = Vec::with_capacity(dimensions.0);
        stack.push((start_point, 0, node_map[start_point].unwrap()));
        while let Some((current, mut distance, mut src)) = stack.pop() {
            let neighbors = ALL_S4_DIRECTIONS.iter().filter_map(|&d| {
                grid_translate(current, d.vector(1), &dimensions)
                    .and_then(|c| (!collision_map[c]).then_some(c))
            });

            if neighbors.clone().count() > 2 && node_map[current].is_none() {
                node_map[current] = Some(graph.add_node(()));
            }

            if let Some(dest) = node_map[current] {
                if src != dest && graph.find_edge(src, dest).is_none() {
                    graph.add_edge(src, dest, distance);
                }
                distance = 0;
                src = dest;
            }

            if visited[current] {
                continue;
            }
            visited[current] = true;
            stack.extend(neighbors.map(|n| (n, distance + 1, src)));
        }
        graph
    }

    fn find_largest_distance(graph: &UnGraph<(), usize, usize>) -> usize {
        #[derive(Debug, Clone)]
        struct Path {
            sequence: Vec<usize>,
            distance: usize,
        }
        let mut memo = vec![None; graph.node_count()];
        let mut stack = Vec::new();
        stack.push(Path {
            sequence: vec![END_INDEX],
            distance: 0,
        });
        while let Some(path) = stack.pop() {
            let head = *path.sequence.last().unwrap();
            for edge in graph.edges(head.into()) {
                let target = edge.target().index();
                if path.sequence.contains(&target) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.sequence.push(target);
                new_path.distance = path.distance + edge.weight();
                if memo[target]
                    .as_ref()
                    .map(|p: &Path| new_path.distance > p.distance)
                    .unwrap_or(true)
                {
                    memo[target] = Some(new_path.clone());
                }
                stack.push(new_path);
            }
        }
        memo.swap_remove(START_INDEX).unwrap().distance
    }

    let graph = parse_graph(input);
    find_largest_distance(&graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1(SAMPLE_SRC), 94);
    }

    #[test]
    pub fn part2_sample() {
        assert_eq!(part2(SAMPLE_SRC), 154);
    }
}
