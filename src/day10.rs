pub fn parse(v: Vec<String>) -> impl Iterator<Item = (usize, i32)> {
    v.into_iter()
        .flat_map(|line| {
            let mut s = line.split_whitespace();
            match s.next() {
                Some("noop") => vec![0],
                Some("addx") => vec![0, s.next().unwrap().parse().unwrap()],
                _ => panic!("uhoh {:?}", line),
            }
            .into_iter()
        })
        .enumerate()
}

pub fn part1(v: Vec<String>) -> i32 {
    let arr = [20, 60, 100, 140, 180, 220];

    parse(v)
        .fold((1, 0, 0), |(x, strength, i), (c, y)| {
            if i < arr.len() && c as i32 + 1 == arr[i] {
                (x + y, strength + arr[i] * x, i + 1)
            } else {
                (x + y, strength, i)
            }
        })
        .1
}

pub fn part2(v: Vec<String>) {
    let screen = parse(v)
        .fold((1, vec![vec!['.'; 40]; 6]), |(x, mut screen), (c, y)| {
            if (x - (c % 40) as i32).abs() <= 1 {
                screen[c / 40][c % 40] = '#'
            }
            (x + y, screen)
        })
        .1;

    for v in screen {
        println!("{}", v.iter().map(|x| *x as char).collect::<String>());
    }
}
