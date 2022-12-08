use itertools::Itertools;

fn parse(v: Vec<String>) -> Vec<Vec<i32>> {
    v.iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| *b as i32 - '0' as i32)
                .collect()
        })
        .collect()
}

fn max_scan(acc: &mut i32, ele: &i32) -> Option<i32> {
    let ret = *acc;
    *acc = std::cmp::max(*acc, *ele);
    Some(ret)
}

pub fn part1(v: Vec<String>) -> usize {
    let trees = parse(v);

    let left: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| row.iter().scan(-1, max_scan).collect_vec())
        .collect_vec();

    let right = trees
        .iter()
        .map(|row| {
            row.iter()
                .rev()
                .scan(-1, max_scan)
                .collect_vec()
                .into_iter()
                .rev()
                .collect_vec()
        })
        .collect_vec();

    let up = trees
        .iter()
        .scan(vec![-1; trees[0].len()], |acc, row| {
            Some(
                acc.iter_mut()
                    .zip(row)
                    .map(|(acc, tree)| max_scan(acc, tree).unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let down = trees
        .iter()
        .rev()
        .scan(vec![-1; trees[0].len()], |acc, row| {
            Some(
                acc.iter_mut()
                    .zip(row)
                    .map(|(acc, tree)| max_scan(acc, tree).unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    let is_visible = |i: usize, j: usize, t: i32| {
        t > left[i][j] || t > right[i][j] || t > up[i][j] || t > down[i][j]
    };

    trees.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|(j, t)| is_visible(i, *j, **t))
            .count()
    })
}

fn distance(r1: i32, r2: i32, c1: i32, c2: i32, trees: &[Vec<i32>], height: i32) -> usize {
    let iter = num::range_step(r1, r2, if r1 > r2 { -1 } else { 1 })
        .cartesian_product(num::range_step(c1, c2, if c1 > c2 { -1 } else { 1 }))
        .map(|(x, y)| trees[x as usize][y as usize]);

    let d = iter.clone().count();
    std::cmp::min(iter.take_while(|t| height > *t).count() + 1, d)
}

pub fn part2(v: Vec<String>) -> usize {
    let trees = parse(v);

    (1..trees.len() as i32 - 1)
        .cartesian_product(1..trees[0].len() as i32 - 1)
        .map(|(x, y)| {
            let height = trees[x as usize][y as usize];
            distance(x + 1, trees.len() as i32, y, y + 1, &trees, height)
                * distance(x, x + 1, y + 1, trees[0].len() as i32, &trees, height)
                * distance(x - 1, -1, y, y + 1, &trees, height)
                * distance(x, x + 1, y - 1, -1, &trees, height)
        })
        .max()
        .unwrap()
}
