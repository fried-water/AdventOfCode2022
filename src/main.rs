pub mod file;

pub mod day1;
pub mod day2;

fn string_wrap<F: std::fmt::Debug>(
    f: impl Fn(Vec<String>) -> F + 'static,
) -> Box<dyn Fn(Vec<String>) -> String> {
    Box::new(move |v| format!("{:?}", f(v)))
}

fn problems() -> Vec<Vec<Box<dyn Fn(Vec<String>) -> String>>> {
    vec![
        vec![string_wrap(day1::part1), string_wrap(day1::part2)],
        vec![string_wrap(day2::part1), string_wrap(day2::part2)],
    ]
}

fn main() {
    let mut args = std::env::args();

    args.next();

    let day: usize = args
        .next()
        .expect("Expect day argument")
        .parse()
        .expect("Expect day argument");
    let part: usize = args
        .next()
        .expect("Expect part argument")
        .parse()
        .expect("Expect part argument");
    let file = args.next().expect("Expect file argument");

    let p = problems();
    let func = p
        .get(day - 1)
        .expect("Invalid day")
        .get(part - 1)
        .expect("Invalid part");

    if let Ok(lines) = file::read_lines(&file) {
        println!("{}", func(lines))
    } else {
        println!("Error reading file {}", &file);
    }
}
