use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use adventofcode2019::build_main;

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines()
        .for_each(|line| {
            let parts = line.split(")").collect_vec();
            graph.entry(parts[0]).or_default().push(parts[1]);
            graph.entry(parts[1]).or_default().push(parts[0]);
        });

    graph
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);

    let mut depths = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(("COM", 0));
    depths.insert("COM", 0);

    while let Some((name, depth)) = queue.pop_front() {
        for &nbr in graph[&name].iter() {
            if !depths.contains_key(nbr) {
                depths.insert(nbr, depth + 1);
                queue.push_back((nbr, depth + 1));
            }
        }
    }

    depths.values().sum::<usize>()
}

fn part2(input: &str) -> usize {
    let graph = parse_graph(input);

    let mut seen = HashSet::new();
    seen.insert("YOU");

    let mut queue = VecDeque::new();
    queue.push_back(("YOU", 0));

    while let Some((name, dist)) = queue.pop_front() {
        for &nbr in graph[&name].iter() {
            if nbr == "SAN" {
                return dist + 1 - 2;
            }
            if seen.insert(nbr) {
                queue.push_back((nbr, dist + 1));
            }
        }
    }

    unreachable!()
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1), 42);
    }

    const TEST_INPUT2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 4);
    }
}