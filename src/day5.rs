use itertools::Itertools;

pub fn parse(v: Vec<String>) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let height = v.iter().take_while(|l| l.contains('[')).count();
    let count = v[height].split_whitespace().count();

    let buckets = v[..height]
        .iter()
        .rev()
        .fold(vec![Vec::<char>::new(); count], |mut acc, v| {
            acc.iter_mut()
                .zip(
                    v.chars()
                        .chunks(4)
                        .into_iter()
                        .map(|mut z| z.nth(1).unwrap()),
                )
                .for_each(|(s, c)| {
                    if c != ' ' {
                        s.push(c)
                    }
                });
            acc
        });

    let cmds: Vec<(usize, usize, usize)> = v[height + 2..]
        .iter()
        .map(|s| {
            s.split(' ')
                .filter_map(|x| x.parse().ok())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect();

    (buckets, cmds)
}

pub fn part1(v: Vec<String>) -> String {
    let (buckets, cmds) = parse(v);

    cmds.iter()
        .fold(buckets, |mut acc, (c, f, t)| {
            let src = &mut acc.get_mut(f - 1).unwrap();
            let mut to_move = src.drain(src.len() - c..).rev().collect_vec();
            acc.get_mut(t - 1).unwrap().append(&mut to_move);
            acc
        })
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

pub fn part2(v: Vec<String>) -> String {
    let (buckets, cmds) = parse(v);

    cmds.iter()
        .fold(buckets, |mut acc, (c, f, t)| {
            let src = &mut acc.get_mut(f - 1).unwrap();
            let mut to_move = src.drain(src.len() - c..).collect_vec();
            acc.get_mut(t - 1).unwrap().append(&mut to_move);
            acc
        })
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}
