use itertools::Itertools;
use regex::Regex;

use glam::{ivec2, IVec2};

use std::collections::HashSet;

fn manhattan(x: IVec2, y: IVec2) -> i32 {
    let d = (x - y).abs();
    d.x + d.y
}

fn parse(v: Vec<String>) -> (Vec<(IVec2, i32)>, HashSet<IVec2>) {
    let regex =
        Regex::new(r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#)
            .unwrap();

    let parsed = v
        .iter()
        .map(|l| {
            regex
                .captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .map(|(x1, y1, x2, y2)| (ivec2(x1, y1), ivec2(x2, y2)))
        .collect_vec();

    (
        parsed
            .iter()
            .map(|(s, b)| (*s, manhattan(*s, *b)))
            .collect(),
        parsed.iter().map(|(_, b)| *b).collect(),
    )
}

fn ranges(sensors: &[(IVec2, i32)], row: i32) -> impl Iterator<Item = (i32, i32)> + Clone {
    sensors
        .iter()
        .map(|(s, m)| (s.x, m - (row - s.y).abs()))
        .filter(|(_, d)| *d >= 0)
        .map(|(x, d)| (x - d, x + d))
        .sorted()
}

fn slot(sensors: &[(IVec2, i32)], row: i32) -> Option<i32> {
    ranges(sensors, row)
        .fold((0, None), |(e, col), (s1, e1)| {
            if col.is_some() {
                (e, col)
            } else if s1 - e == 2 {
                (e, Some(e + 1))
            } else {
                (e.max(e1), None)
            }
        })
        .1
}

pub fn part1(v: Vec<String>) -> usize {
    const ROW: i32 = 2000000;
    // const ROW: i32 = 10;

    let (sensors, beacons) = parse(v);

    ranges(&sensors, ROW)
        .fold(HashSet::new(), |acc, (s, e)| {
            (s..=e).fold(acc, |mut acc, x| {
                acc.insert(ivec2(x, ROW));
                acc
            })
        })
        .into_iter()
        .filter(|p| !beacons.contains(p))
        .count()
}

pub fn part2(v: Vec<String>) -> Option<((i32, i32), usize)> {
    const LIMIT: i32 = 4000000;
    // const LIMIT:i32 = 20;

    let (sensors, _) = parse(v);

    (0..=LIMIT).find_map(|row| {
        slot(&sensors, row)
            .map(|col| (col, row))
            .map(|(col, row)| ((col, row), (col as usize * LIMIT as usize + row as usize)))
    })
}
