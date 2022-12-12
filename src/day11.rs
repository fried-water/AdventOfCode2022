use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: Vec<usize>,
    op: (bool, Option<usize>),
    div_by: usize,
    if_true: usize,
    if_false: usize,
}

fn parse_monkey(c: regex::Captures) -> Monkey {
    let mut iter = c.iter().skip(1).map(|c| c.unwrap().as_str());
    Monkey {
        items: iter
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect(),
        op: {
            let mut s = iter.next().unwrap().split(' ').skip(1);
            (s.next().unwrap() == "+", s.next().unwrap().parse().ok())
        },
        div_by: iter.next().unwrap().parse().unwrap(),
        if_true: iter.next().unwrap().parse().unwrap(),
        if_false: iter.next().unwrap().parse().unwrap(),
    }
}

fn simulate(
    mut counts: Vec<usize>,
    mut monkies: Vec<Monkey>,
    divisor: usize,
) -> (Vec<usize>, Vec<Monkey>) {
    let max_modulo: usize = [2, 3, 5, 7, 9, 11, 13, 17, 19, 23].iter().product();

    for m in 0..monkies.len() {
        for i in 0..monkies[m].items.len() {
            let old = monkies[m].items[i];

            let (is_plus, term) = monkies[m].op;

            let new = if is_plus {
                old + term.unwrap_or(old)
            } else {
                old * term.unwrap_or(old)
            } / divisor;

            let target = if new % monkies[m].div_by == 0 {
                monkies[m].if_true
            } else {
                monkies[m].if_false
            };

            monkies[target].items.push(new % max_modulo);
        }

        counts[m] += monkies[m].items.len();
        monkies[m].items.clear();
    }

    (counts, monkies)
}

fn solve(v: Vec<String>, iters: usize, divisor: usize) -> usize {
    const MONKEY_REGEX: &str = r#"Monkey [0-9]+:
  Starting items: (.*)
  Operation: new = (.*)
  Test: divisible by ([0-9]+)
    If true: throw to monkey ([0-9]+)
    If false: throw to monkey ([0-9]+)"#;

    let monkies = Regex::new(MONKEY_REGEX)
        .unwrap()
        .captures_iter(&v.join("\n"))
        .map(parse_monkey)
        .collect_vec();

    (0..iters)
        .fold((vec![0; monkies.len()], monkies), |(counts, monkies), _| {
            simulate(counts, monkies, divisor)
        })
        .0
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

pub fn part1(v: Vec<String>) -> usize {
    solve(v, 20, 3)
}

pub fn part2(v: Vec<String>) -> usize {
    solve(v, 10000, 1)
}
