use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{algo::dijkstra, graph::NodeIndex, visit::Dfs, Directed, Graph};

pub struct Input {
    width: usize,
    height: usize,
    start: NodeIndex<usize>,
    graph: Graph<(), (), Directed, usize>,
    matrix: Vec<char>,
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let matrix: Vec<char> = input
        .lines()
        .flat_map(str::lines)
        .flat_map(str::chars)
        .collect();
    let start = matrix
        .iter()
        .copied()
        .position(|c| c == 'S')
        .unwrap()
        .into();
    let get_adjacent = |i: usize, (j, k): (isize, isize)| {
        i.checked_add_signed(j * (width as isize) + k)
            .and_then(|i| (i < matrix.len()).then_some(i))
    };
    let mut graph = Graph::from_edges(
        matrix
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, c)| !['S', '.'].contains(c))
            .map(|(i, c)| {
                (
                    i,
                    match c {
                        '|' => [(-1, 0), (1, 0)],
                        '-' => [(0, -1), (0, 1)],
                        'L' => [(-1, 0), (0, 1)],
                        'J' => [(-1, 0), (0, -1)],
                        '7' => [(0, -1), (1, 0)],
                        'F' => [(0, 1), (1, 0)],
                        c => unreachable!("{}", c),
                    },
                )
            })
            .map(|(i, n)| (i, n.map(|n| get_adjacent(i, n))))
            .flat_map(|(i, n)| n.into_iter().flat_map(move |n| n.map(|n| (i, n)))),
    );
    let start_neighbors = graph.neighbors_undirected(start).collect::<Vec<_>>();
    for target in start_neighbors {
        graph.add_edge(start, target, ());
    }
    Input {
        width,
        height,
        start,
        graph,
        matrix,
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> usize {
    let distances = dijkstra(&input.graph, input.start, None, |_| 1);
    *distances.values().max().unwrap()
}

const WEST_ADJACENT_NODES: [char; 3] = ['-', 'J', '7'];
const EAST_ADJACENT_NODES: [char; 3] = ['-', 'F', 'L'];
const SOUTH_ADJACENT_NODES: [char; 3] = ['|', '7', 'F'];
const NORTH_ADJACENT_NODES: [char; 3] = ['|', 'J', 'L'];

#[aoc(day10, part2)]
pub fn part2(
    Input {
        width,
        height,
        start,
        graph,
        matrix,
    }: &Input,
) -> usize {
    let width = *width;
    let height = *height;
    let start = *start;
    let mut point_traverse = Dfs::new(&graph, start);
    let mut loop_nodes = Vec::new();
    while let Some(point) = point_traverse.next(&graph) {
        loop_nodes.push(point.index());
    }
    let mut clean_matrix = matrix.clone();
    for (_, c) in clean_matrix
        .iter_mut()
        .enumerate()
        .filter(|(i, _)| !loop_nodes.contains(i))
    {
        *c = '.';
    }
    let start = start.index();
    let neighboring_nodes = (
        clean_matrix[start - 1],
        clean_matrix[start - width],
        clean_matrix[start + 1],
        clean_matrix[start + width],
    ); // WNES
    clean_matrix[start] = match (
        EAST_ADJACENT_NODES.contains(&clean_matrix[start - 1]),
        SOUTH_ADJACENT_NODES.contains(&clean_matrix[start - width]),
        WEST_ADJACENT_NODES.contains(&clean_matrix[start + 1]),
        NORTH_ADJACENT_NODES.contains(&clean_matrix[start + width]),
    ) {
        (false, true, false, true) => '|',
        (true, false, true, false) => '-',
        (false, true, true, false) => 'L',
        (true, true, false, false) => 'J',
        (true, false, false, true) => '7',
        (false, false, true, true) => 'F',
        p => unreachable!("{:?} {:?}", neighboring_nodes, p),
    };
    let mut area_inside = 0;
    for y in 0..height {
        let row_start = y * width;
        for x in 0..width {
            let index = row_start + x;
            let is_inside = if clean_matrix[index] == '.' {
                let mut count = 0;
                let mut edge_start = '.';
                for i in row_start..=row_start + x {
                    let c = clean_matrix[i];
                    match (c, edge_start) {
                        ('|', _) => count += 1,
                        ('F', _) => edge_start = 'F',
                        ('L', _) => edge_start = 'L',
                        ('J', 'F') => count += 1,
                        ('J', 'L') => count += 2,
                        ('7', 'F') => count += 2,
                        ('7', 'L') => count += 1,
                        ('.' | '-', _) => {}
                        _ => unreachable!("{}, {}", c, edge_start),
                    }
                }
                count % 2 == 1
            } else {
                false
            };
            print!("{}", if is_inside { 'X' } else { clean_matrix[index] });
            area_inside += is_inside as usize;
        }
        println!();
    }
    area_inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(&generator(input)), 4);

        let input = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(&generator(input)), 4);
    }
}
