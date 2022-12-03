use itertools::Itertools;

fn group(v: Vec<String>) -> Vec<i32> {
    v.into_iter()
        .group_by(|el| !el.is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.map(|e| e.parse::<i32>().unwrap()).sum())
        .collect()
}

pub fn part1(v: Vec<String>) -> i32 {
    *group(v).iter().max().unwrap()
}

pub fn part2(v: Vec<String>) -> i32 {
    group(v)
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .into_iter()
        .take(3)
        .sum()
}
