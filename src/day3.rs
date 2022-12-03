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

fn add(mut x: Sack, y: Sack) -> Sack {
    for (x, y) in x.iter_mut().zip(y) {
        *x += y
    }
    x
}

pub fn part1(v: Vec<String>) -> i32 {
    v.iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| add(create_sack(l), create_sack(r)))
        .map(|s| threshold(s, 2) + 1)
        .sum()
}

pub fn part2(v: Vec<String>) -> i32 {
    v.chunks(3)
        .into_iter()
        .map(|sacks| sacks.iter().map(|s| create_sack(s)).fold([0; 52], add))
        .map(|s| threshold(s, 3) + 1)
        .sum()
}
