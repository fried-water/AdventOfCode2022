use itertools::Itertools;

fn parse(line: &str) -> (i32, i32, i32, i32) {
    line.split([',', '-'])
        .map(|x| x.parse().unwrap())
        .collect_tuple::<(_, _, _, _)>()
        .unwrap()
}

pub fn part1(v: Vec<String>) -> usize {
    v.iter()
        .map(|l| parse(l))
        .filter(|(a, b, x, y)| (a >= x && b <= y) || (x >= a && y <= b))
        .count()
}

pub fn part2(v: Vec<String>) -> usize {
    v.iter()
        .map(|l| parse(l))
        .filter(|(a, b, x, y)| (x >= a && x <= b) || (a >= x && a <= y))
        .count()
}
