fn split2(s: &str, c: char) -> (&str, &str) {
    let mut v = s.split(c);
    (v.next().unwrap(), v.next().unwrap())
}

fn range((x, y): (&str, &str)) -> (i32, i32) {
    (x.parse().unwrap(), y.parse().unwrap())
}

fn contains_range((f1, l1): (i32, i32), (f2, l2): (i32, i32)) -> bool {
    f1 >= f2 && l1 <= l2
}

fn contains((f, l): (i32, i32), x: i32) -> bool {
    x >= f && x <= l
}

pub fn part1(v: Vec<String>) -> usize {
    v.iter()
        .map(|line| split2(line, ','))
        .map(|(a, b)| (range(split2(a, '-')), range(split2(b, '-'))))
        .filter(|(a, b)| contains_range(*a, *b) || contains_range(*b, *a))
        .count()
}

pub fn part2(v: Vec<String>) -> usize {
    v.iter()
        .map(|line| split2(line, ','))
        .map(|(a, b)| (range(split2(a, '-')), range(split2(b, '-'))))
        .filter(|(a, b)| contains(*a, b.0) || contains(*b, a.0))
        .count()
}
