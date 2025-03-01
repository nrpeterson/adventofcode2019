use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use adventofcode2019::build_main;
use adventofcode2019::math::gcd;
use adventofcode2019::points::Point2D;

struct Space {
    asteroids: HashSet<Point2D>
}

fn parse_input(input: &str) -> Space {
    let mut asteroids = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Point2D(x as isize, y as isize));
            }
        }
    }

    Space { asteroids }
}

fn reduce(point: Point2D) -> Point2D {
    let Point2D(x, y) = point;
    let d = gcd(x, y).abs();
    Point2D(x / d, y / d)
}

fn visible_from(space: &Space, point: Point2D) -> usize {
    space.asteroids.iter()
        .filter(|&&b| b != point)
        .map(|&b| b - point)
        .map(reduce)
        .unique()
        .count()
}

fn angle_to_y(point: Point2D) -> f32 {
    -((point.0 as f32).atan2(point.1 as f32))
}

fn rays(space: &Space, from: Point2D) -> Vec<Vec<Point2D>> {
    let mut rays: HashMap<Point2D, Vec<(isize, Point2D)>> = HashMap::new();
    for &asteroid in space.asteroids.iter() {
        if asteroid == from {
            continue;
        }
        let delta = asteroid - from;
        let d = reduce(delta);
        let k = if delta.0 != 0 { delta.0 / d.0 } else { delta.1 / d.1 };

        rays.entry(d).or_default().push((k, asteroid));
    }

    rays.values_mut().for_each(|r| r.sort_by_key(|(k, _)| -*k));

    rays.into_iter()
        .sorted_by(|&(a, _), &(b, _)| {
            angle_to_y(a).partial_cmp(&angle_to_y(b)).unwrap()
        })
        .map(|(_, v)| {
            v.into_iter()
                .map(|(_, point)| point)
                .collect()
        })
        .collect_vec()

}

fn find_station(space: &Space) -> (Point2D, usize) {
    space.asteroids.iter()
        .map(|&a| (a, visible_from(space, a)))
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn part1(input: &str) -> usize {
    let space = parse_input(input);
    find_station(&space).1
}

fn part2(input: &str) -> isize {
    let space = parse_input(input);
    let (station, _) = find_station(&space);
    let mut rays = rays(&space, station);

    let mut deleted = 0;
    let mut last_deleted = 0;

    loop {
        for i in 0..rays.len() {
            if deleted == 200 {
                return last_deleted;
            }
            if let Some(point) = rays[i].pop() {
                deleted += 1;
                last_deleted = 100*point.0 + point.1;
            }
        }
    }
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn test_part1() {
        let space = parse_input(TEST_INPUT);
        assert_eq!(find_station(&space), (Point2D(11, 13), 210));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 802);
    }
}