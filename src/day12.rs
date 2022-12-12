use glam::{ivec2, IVec2};
use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn parse(v: Vec<String>) -> (Vec<Vec<i32>>, IVec2, IVec2) {
    v.into_iter().enumerate().fold(
        (Vec::new(), IVec2::ZERO, IVec2::ZERO),
        |(mut board, mut start, mut end), (row, line)| {
            board.push(
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' => {
                            start = ivec2(col as i32, row as i32);
                            0
                        }
                        'E' => {
                            end = ivec2(col as i32, row as i32);
                            25
                        }
                        'a'..='z' => c as i32 - 'a' as i32,
                        _ => panic!("invalid char {}", c),
                    })
                    .collect(),
            );
            (board, start, end)
        },
    )
}

fn height(b: &Vec<Vec<i32>>, p: IVec2) -> i32 {
    *b.get(p.y as usize)
        .and_then(|row| row.get(p.x as usize))
        .unwrap_or(&30)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapElement {
    distance: i32,
    p: IVec2,
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn route(b: &Vec<Vec<i32>>, s: Vec<IVec2>, end: IVec2) -> Option<i32> {
    let mut heap = s
        .into_iter()
        .map(|s| HeapElement { distance: 0, p: s })
        .collect::<BinaryHeap<HeapElement>>();

    let mut visited = HashSet::new();

    while let Some(HeapElement { distance, p }) = heap.pop() {
        if p == end {
            return Some(distance);
        } else if visited.insert(p) {
            let h = height(b, p);
            [p + IVec2::X, p - IVec2::X, p + IVec2::Y, p - IVec2::Y]
                .into_iter()
                .filter(|p2| height(b, *p2) <= h + 1)
                .for_each(|p2| {
                    heap.push(HeapElement {
                        distance: distance + 1,
                        p: p2,
                    })
                })
        }
    }

    None
}

pub fn part1(v: Vec<String>) -> Option<i32> {
    let (board, start, end) = parse(v);
    route(&board, vec![start], end)
}

pub fn part2(v: Vec<String>) -> Option<i32> {
    let (board, _, end) = parse(v);
    let starts = (0..board.len())
        .cartesian_product(0..board[0].len())
        .map(|(y, x)| ivec2(x as i32, y as i32))
        .filter(|p| height(&board, *p) == 0)
        .collect_vec();
    route(&board, starts, end)
}
