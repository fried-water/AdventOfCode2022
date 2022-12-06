use itertools::Itertools;

pub fn unique_subrange(s: &str, i: usize) -> usize {
  s.as_bytes().windows(i)
    .enumerate()
    .find(|(_, w)| w.iter().unique().count() == i)
    .unwrap().0 + i
}

pub fn part1(v: Vec<String>) -> usize { unique_subrange(v.first().unwrap(), 4) }
pub fn part2(v: Vec<String>) -> usize { unique_subrange(v.first().unwrap(), 14) }
