use itertools::Itertools;
use std::collections::HashSet;

fn mv((x, y): (i32, i32), d: char) -> (i32, i32) {
    match d {
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _ => panic!("uhoh"),
    }
}

fn follow((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    let dx = x1 - x2;
    let dy = y1 - y2;

    if num::abs(dx) < 2 && num::abs(dy) < 2 {
        (x2, y2)
    } else {
        (x2 + num::signum(dx), y2 + num::signum(dy))
    }
}

fn run(v: Vec<String>, length: usize) -> usize {
    v.into_iter()
        .map(|line| {
            line.split(' ')
                .map(|x| x.to_string())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .map(|(f, l)| (f.chars().next().unwrap(), l.parse::<i32>().unwrap()))
        .fold(
            (HashSet::<(i32, i32)>::new(), vec![(0, 0); length]),
            |(history, s), (d, c)| {
                (0..c).fold((history, s), |(mut history, mut s), _| {
                    s[0] = mv(s[0], d);
                    for i in 1..s.len() {
                        s[i] = follow(s[i - 1], s[i]);
                    }
                    history.insert(*s.last().unwrap());
                    (history, s)
                })
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
