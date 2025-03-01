use adventofcode2019::build_main;
use adventofcode2019::math::lcm;
use adventofcode2019::parsers::isize_str;
use adventofcode2019::points::Point3D;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, space0};
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use std::collections::HashSet;

struct Moon {
    position: Point3D,
    velocity: Point3D
}

struct Space {
    moons: Vec<Moon>
}

impl Space {
    fn parse(input: &str) -> Space {
        fn point(input: &str) -> IResult<&str, Point3D> {
            map(
                delimited(
                    char('<'),
                    tuple((
                        preceded(pair(tag("x="), space0), isize_str),
                        preceded(pair(tag(", y="), space0), isize_str),
                        preceded(pair(tag(", z="), space0), isize_str)
                    )),
                    char('>')
                ),
                |(x, y, z)| Point3D(x, y, z)
            )(input)
        }

        all_consuming(
            map(
                separated_list1(newline, point),
                |positions: Vec<Point3D>| {
                    let moons = positions.into_iter()
                        .map(|position| Moon { position, velocity: Point3D(0, 0, 0) })
                        .collect_vec();
                    Space { moons }
                }
            )
        )(input).unwrap().1
    }

    fn step(&mut self) {
        let mut changes = vec![Point3D(0, 0, 0); self.moons.len()];
        for (i, j) in (0..self.moons.len()).tuple_combinations() {
            let Point3D(x0, y0, z0) = self.moons[i].position;
            let Point3D(x1, y1, z1) = self.moons[j].position;

            if x0 < x1 {
                changes[i].0 += 1;
                changes[j].0 -= 1;
            }
            if x0 > x1 {
                changes[i].0 -= 1;
                changes[j].0 += 1;
            }
            if y0 < y1 {
                changes[i].1 += 1;
                changes[j].1 -= 1;
            }
            if y0 > y1 {
                changes[i].1 -= 1;
                changes[j].1 += 1;
            }
            if z0 < z1 {
                changes[i].2 += 1;
                changes[j].2 -= 1;
            }
            if z0 > z1 {
                changes[i].2 -= 1;
                changes[j].2 += 1;
            }
        }

        self.moons.iter_mut().zip(changes).for_each(|(m, d)| {
            m.velocity += d;
        });

        self.moons.iter_mut().for_each(|m| m.position += m.velocity);
    }

    fn coord_states(&self) -> Vec<Vec<(isize, isize)>> {
        let mut result = Vec::new();

        for f in [|p: Point3D| { p.0 }, |p: Point3D| { p.1 }, |p: Point3D| { p.2 }] {
            result.push(
                self.moons.iter()
                    .map(|m| (f(m.position), f(m.velocity)))
                    .collect_vec()
            );
        }

        result
    }
}

fn part1(input: &str) -> isize {
    let mut space = Space::parse(input);
    (0..1000).for_each(|_| space.step());

    space.moons.into_iter()
        .map(|moon| moon.position.l1_norm() * moon.velocity.l1_norm())
        .sum()
}

fn part2(input: &str) -> isize {
    let mut space = Space::parse(input);
    let mut cycle_times = vec![None; 3];

    let mut seen: Vec<HashSet<Vec<(isize, isize)>>> = vec![HashSet::new(); 3];

    for i in 0.. {
        for (dim, coord_state) in space.coord_states().into_iter().enumerate() {
            if !seen[dim].insert(coord_state) {
                cycle_times[dim] = cycle_times[dim].or(Some(i));
            }
        }
        if cycle_times.iter().all(|t| t.is_some()) { break; }
        space.step();
    }

    let times = cycle_times.into_iter().map(|t| t.unwrap()).collect_vec();
    let t0 = times[0];
    let t1 = times[1];
    let t2 = times[2];

    lcm(lcm(t0, t1), t2)
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);