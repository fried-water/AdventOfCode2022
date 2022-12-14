use glam::IVec2;
use std::collections::HashSet;

fn follow(h: IVec2, t: IVec2) -> IVec2 {
    if (h - t).abs().max_element() > 1 {
        (h - t).signum()
    } else {
        IVec2::ZERO
    }
}

fn run(v: Vec<String>, length: usize) -> usize {
    v.into_iter()
        .flat_map(|line| {
            std::iter::repeat(match line.chars().next().unwrap() {
                'U' => IVec2::Y,
                'D' => -IVec2::Y,
                'L' => -IVec2::X,
                'R' => IVec2::X,
                _ => panic!("uhoh"),
            })
            .take(line.chars().skip(2).collect::<String>().parse().unwrap())
        })
        .fold(
            (HashSet::<IVec2>::new(), vec![IVec2::ZERO; length]),
            |(mut history, mut s), d| {
                s[0] += d;
                for i in 1..s.len() {
                    s[i] = s[i] + follow(s[i - 1], s[i]);
                }
                history.insert(*s.last().unwrap());
                (history, s)
            },
        )
        .0
        .len()
}

pub fn part1(v: Vec<String>) -> usize {
    run(v, 2)
}
pub fn part2(v: Vec<String>) -> usize {
    run(v, 10)
}
