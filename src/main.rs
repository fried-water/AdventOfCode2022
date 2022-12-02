use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().into_iter()
        .filter_map(|x| x.ok())
        .collect())
}

fn group(v: Vec<String>) -> Vec<i32> {
    v.into_iter()
        .group_by(|el| !el.is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.map(|e| e.parse::<i32>().unwrap()).sum())
        .collect()
}

fn part1(v: Vec<String>) -> i32 {
    group(v).into_iter().max().unwrap()
}

fn part2(v: Vec<String>) -> i32 {
    group(v).into_iter()
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .into_iter()
        .take(3)
        .sum()
}

fn main() {
    let mut args = env::args();

    args.next();

    let part = args.next().expect("Expect part argument").parse::<i32>().expect("Expect part argument");
    let file = args.next().expect("Expect file argument");

    if let Ok(lines) = read_lines(&file) {
        if part == 1 {
            println!("{:?}", part1(lines));
        } else if part == 2 {
            println!("{:?}", part2(lines));
        } else {
            println!("Expected part 1 or 2");
        }
    } else {
        println!("Error reading file {}", &file);
    }
}
