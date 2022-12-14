use glam::{ivec2, IVec2};

use std::collections::HashSet;

use crate::iters::cartesian_inclusive;

fn parse_ivec2(s: &str) -> IVec2 {
    let mut iter = s.split(',').map(|x| x.parse::<i32>().unwrap());
    ivec2(iter.next().unwrap(), iter.next().unwrap())
}

fn parse_cave(v: Vec<String>) -> (HashSet<IVec2>, i32) {
    let cave = v
        .iter()
        .flat_map(|l| {
            let iter = l.split(" -> ").map(parse_ivec2);
            iter.clone().zip(iter.skip(1))
        })
        .fold(HashSet::new(), |acc, (start, end)| {
            cartesian_inclusive(start, end).fold(acc, |mut acc, p| {
                acc.insert(p);
                acc
            })
        });

    let ymax = cave.iter().map(|v| v.y).max().unwrap();

    (cave, ymax)
}

const START: IVec2 = ivec2(500, 0);
const ORDER: [IVec2; 3] = [ivec2(0, 1), ivec2(-1, 1), ivec2(1, 1)];

fn simulate(cave: &HashSet<IVec2>, ymax: i32) -> Option<IVec2> {
    let mut pos = START;

    while pos.y < ymax {
        if let Some(next) = ORDER
            .iter()
            .map(|offset| pos + *offset)
            .find(|p| !cave.contains(p))
        {
            pos = next
        } else {
            return Some(pos);
        }
    }

    None
}

pub fn part1(v: Vec<String>) -> usize {
    let (mut cave, ymax) = parse_cave(v);

    let mut count = 0;

    while let Some(pos) = simulate(&cave, ymax) {
        count += 1;
        cave.insert(pos);
    }

    count
}

fn simulate2(cave: &HashSet<IVec2>, ymax: i32) -> IVec2 {
    let mut pos = START;

    loop {
        if let Some(next) = ORDER
            .iter()
            .map(|offset| pos + *offset)
            .find(|p| !cave.contains(p) && p.y < ymax + 2)
        {
            pos = next
        } else {
            return pos;
        }
    }
}

pub fn part2(v: Vec<String>) -> usize {
    let (mut cave, ymax) = parse_cave(v);

    let mut count = 1;
    let mut pos = simulate2(&cave, ymax);

    while pos != ivec2(500, 0) {
        count += 1;
        cave.insert(pos);
        pos = simulate2(&cave, ymax);
    }

    count
}
