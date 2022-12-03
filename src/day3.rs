use itertools::Itertools;

type Sack = [i32; 52];

fn idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as usize) - ('a' as usize)
    } else {
        (c as usize) - ('A' as usize) + 26
    }
}

fn create_sack(s: &str) -> Sack {
    s.chars().fold([0; 52], |mut acc, c| {
        acc[idx(c)] = 1;
        acc
    })
}

fn threshold(x: Sack, t: i32) -> i32 {
    x.into_iter().enumerate().find(|(_, x)| *x == t).unwrap().0 as i32
}

fn add(x: Sack, y: Sack) -> Sack {
    let mut r = [0; 52];
    for (i, (x, y)) in std::iter::zip(x, y).enumerate() {
        r[i] = x + y
    }
    r
}

pub fn part1(v: Vec<String>) -> i32 {
    v.iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| add(create_sack(l), create_sack(r)))
        .map(|s| threshold(s, 2) + 1)
        .sum()
}

pub fn part2(v: Vec<String>) -> i32 {
    v.into_iter()
        .enumerate()
        .group_by(|(i, _)| i / 3)
        .into_iter()
        .map(|(_, sacks)| sacks.map(|(_, el)| create_sack(&el)).fold([0; 52], add))
        .map(|s| threshold(s, 3) + 1)
        .sum()
}
