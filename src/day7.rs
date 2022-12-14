use itertools::Itertools;

use crate::forest::Forest;

#[derive(Debug)]
enum Node {
    Dir(String),
    File(String, usize),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Dir(n) => n.as_str(),
            Node::File(n, _) => n.as_str(),
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Ls(Vec<Node>),
    Cd(String),
}

fn parse(v: Vec<String>) -> Vec<Cmd> {
    v.iter().fold(Vec::<Cmd>::new(), |mut acc, ele| {
        if ele.starts_with('$') && ele.len() == 4 {
            acc.push(Cmd::Ls(Vec::new()));
        } else if ele.starts_with('$') {
            acc.push(Cmd::Cd(ele.split_whitespace().nth(2).unwrap().to_string()))
        } else if let Cmd::Ls(nodes) = acc.last_mut().unwrap() {
            let (x, y) = ele.split_whitespace().collect_tuple::<(_, _)>().unwrap();
            if x == "dir" {
                nodes.push(Node::Dir(y.to_string()))
            } else {
                nodes.push(Node::File(y.to_string(), x.parse().unwrap()))
            }
        } else {
            panic!("uh oh {}", ele);
        }

        acc
    })
}

fn execute(mut f: Forest<Node>, mut cursor: usize, cmds: Vec<Cmd>) -> Forest<Node> {
    cmds.into_iter().for_each(|cmd| match cmd {
        Cmd::Ls(nodes) => nodes.into_iter().for_each(|n| {
            f.append_child(cursor, n);
        }),
        Cmd::Cd(dir) => match dir.as_str() {
            ".." => cursor = f.parent(cursor).unwrap(),
            "/" => cursor = f.root.unwrap(),
            dir => {
                cursor = f
                    .children(cursor)
                    .find(|(_, _, n)| n.name() == dir)
                    .unwrap()
                    .1
            }
        },
    });

    f
}

pub fn part1(v: Vec<String>) -> usize {
    let mut f = Forest::<Node>::new();
    let root = f.append_root(Node::Dir(String::new()));
    let f = execute(f, root, parse(v));

    f.post_order_root()
        .fold(vec![(false, 0); f.len()], |mut acc, (f, id, n)| {
            acc[id] = match n {
                Node::File(_, size) => (false, *size),
                Node::Dir(_) => (true, f.children(id).map(|(_, c, _)| acc[c].1).sum()),
            };
            acc
        })
        .into_iter()
        .filter(|(is_dir, s)| *is_dir && *s <= 100000)
        .map(|(_, s)| s)
        .sum()
}

pub fn part2(v: Vec<String>) -> usize {
    let mut f = Forest::<Node>::new();
    let root = f.append_root(Node::Dir(String::new()));
    let f = execute(f, root, parse(v));

    let total_space = 70000000;
    let desired_free_space = 30000000;

    let sizes = f
        .post_order_root()
        .fold(vec![0; f.len()], |mut acc, (f, id, n)| {
            acc[id] = match n {
                Node::File(_, size) => *size,
                Node::Dir(_) => f.children(id).map(|(_, c, _)| acc[c]).sum(),
            };
            acc
        });

    let needed_space = desired_free_space - (total_space - sizes[0]);

    sizes
        .into_iter()
        .sorted()
        .find(|s| *s > needed_space)
        .unwrap()
}
