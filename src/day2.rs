fn sub_char(a: char, b: char) -> i32 {
    (a as i32) - (b as i32)
}

fn win_point(o: i32, m: i32) -> i32 {
    if o == m {
        3
    } else if (o + 1) % 3 == m {
        6
    } else {
        0
    }
}

fn select(o: i32, l: char) -> i32 {
    match l {
        'X' => (o + 2) % 3,
        'Y' => o,
        'Z' => (o + 1) % 3,
        _ => panic!("Invalid char"),
    }
}

pub fn part1(v: Vec<String>) -> i32 {
    v.iter()
        .map(|s| s.split(' ').map(|s| s.chars().next().unwrap()))
        .map(|mut s| {
            (
                sub_char(s.next().unwrap(), 'A'),
                sub_char(s.next().unwrap(), 'X'),
            )
        })
        .map(|(o, m)| win_point(o, m) + m + 1)
        .sum()
}

pub fn part2(v: Vec<String>) -> i32 {
    v.iter()
        .map(|s| s.split(' ').map(|s| s.chars().next().unwrap()))
        .map(|mut s| (sub_char(s.next().unwrap(), 'A'), s.next().unwrap()))
        .map(|(o, l)| (o, select(o, l)))
        .map(|(o, m)| win_point(o, m) + m + 1)
        .sum()
}
