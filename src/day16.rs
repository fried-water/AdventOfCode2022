use itertools::Itertools;
use regex::Regex;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Valve = u8;

type State = (i16, (i8, i8), (Valve, Valve), u64);

fn get(bitset: u64, idx: Valve) -> bool {
    bitset & 1 << idx != 0
}

fn parse(v: Vec<String>) -> (Valve, Vec<i16>, Vec<Vec<Valve>>) {
    let regex =
        Regex::new(r#"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#).unwrap();

    let data = v
        .iter()
        .map(|l| {
            regex
                .captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .map(|(name, flow, adjacent)| {
            (
                name,
                flow.parse().unwrap(),
                adjacent.split(", ").collect_vec(),
            )
        })
        .collect_vec();

    let names = data
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, (n, _, _))| {
            acc.insert(n, i as Valve);
            acc
        });

    (
        *names.get(&"AA").unwrap(),
        data.iter().map(|(_, f, _)| *f).collect_vec(),
        data.iter()
            .map(|(_, _, adj)| adj.iter().map(|n| *names.get(n).unwrap()).collect_vec())
            .collect_vec(),
    )
}

fn cost(flows: &[i16], (f, rs, _, opened): &State) -> i16 {
    let r = 2.max(rs.0.max(rs.1));

    flows
        .iter()
        .enumerate()
        .filter(|(i, _)| !get(*opened, *i as Valve))
        .map(|(_, f)| f * (r - 2) as i16)
        .sum::<i16>()
        + *f
}

fn search(
    start_valve: Valve,
    rounds: (i8, i8),
    flows: &[i16],
    neighbours: &[Vec<Valve>],
) -> (i64, Option<i16>) {
    let initial: State = (0, rounds, (start_valve, start_valve), 0);

    let mut heap = BinaryHeap::from([(cost(flows, &initial), initial)]);

    let mut visited = HashSet::new();
    let mut heap_ops = 0;

    while let Some((_, (flow, rs, vs, opened))) = heap.pop() {
        heap_ops += 1;

        if rs == (0, 0) {
            return (heap_ops, Some(flow));
        } else if visited.insert((flow, rs, vs, opened)) {
            let b = rs.0 != rs.1;

            let v = if b { vs.0 } else { vs.1 };
            let remaining = if b { rs.0 } else { rs.1 };

            let next_r = if b {
                (rs.0 - 1, rs.1)
            } else {
                (rs.0, rs.1 - 1)
            };

            if !get(opened, v) && flows[v as usize] > 0 {
                let next = (
                    flow + flows[v as usize] * (remaining - 1) as i16,
                    next_r,
                    vs,
                    opened | 1 << v,
                );
                heap.push((cost(flows, &next), next));
            }

            neighbours[v as usize]
                .iter()
                .map(|adj| {
                    (
                        flow,
                        next_r,
                        if b { (*adj, vs.1) } else { (vs.0, *adj) },
                        opened,
                    )
                })
                .for_each(|ele| heap.push((cost(flows, &ele), ele)));
        }
    }

    (heap_ops, None)
}

pub fn part1(v: Vec<String>) -> (i64, Option<i16>) {
    const ITERS: i8 = 30;
    let (start_valve, flows, neighbours) = parse(v);
    search(start_valve, (ITERS, 0), &flows, &neighbours)
}

pub fn part2(v: Vec<String>) -> (i64, Option<i16>) {
    const ITERS: i8 = 26;
    let (start_valve, flows, neighbours) = parse(v);
    search(start_valve, (ITERS, ITERS), &flows, &neighbours)
}
