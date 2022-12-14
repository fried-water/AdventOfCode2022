use crate::forest::Forest;

use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Node {
    Internal,
    Number(i32),
}

fn parse(s: &str) -> Forest<Node> {
    s.chars()
        .fold(
            (Forest::new(), Vec::new(), String::new()),
            |(mut f, mut s, mut n), c| {
                if c == ',' || c == ']' {
                    if !n.is_empty() {
                        f.append_child(*s.last().unwrap(), Node::Number(n.parse().unwrap()));
                    }
                    n.clear();
                }

                match c {
                    '[' => s.push(f.append(s.last().copied(), Node::Internal)),
                    ']' => {
                        s.pop();
                    }
                    ',' => {}
                    '0'..='9' => n.push(c),
                    _ => panic!("uh oh"),
                }

                (f, s, n)
            },
        )
        .0
}

fn compare(x: &Forest<Node>, y: &Forest<Node>, xid: Option<usize>, yid: Option<usize>) -> Ordering {
    match (xid, yid) {
        (Some(xid), Some(yid)) => {
            let mut cxid = x.first_child(xid);
            let mut cyid = y.first_child(yid);
            match (x[xid], y[yid]) {
                (Node::Internal, Node::Internal) => {
                    let mut ord = compare(x, y, cxid, cyid);

                    while cxid.is_some() && cyid.is_some() && ord == Ordering::Equal {
                        cxid = x.next_sibling(cxid.unwrap());
                        cyid = y.next_sibling(cyid.unwrap());
                        ord = compare(x, y, cxid, cyid);
                    }

                    ord
                }
                (Node::Internal, Node::Number(_)) => {
                    let ord = compare(x, y, cxid, Some(yid));

                    if ord != Ordering::Equal {
                        ord
                    } else {
                        compare(x, y, x.next_sibling(cxid.unwrap()), None)
                    }
                }
                (Node::Number(_), Node::Internal) => {
                    let ord = compare(x, y, Some(xid), cyid);

                    if ord != Ordering::Equal {
                        ord
                    } else {
                        compare(x, y, None, y.next_sibling(cyid.unwrap()))
                    }
                }
                (Node::Number(xn), Node::Number(yn)) => xn.cmp(&yn),
            }
        }
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

pub fn part1(v: Vec<String>) -> usize {
    v.chunks(3)
        .map(|lines| (parse(&lines[0]), parse(&lines[1])))
        .map(|(x, y)| compare(&x, &y, x.root, y.root))
        .enumerate()
        .filter_map(|(i, ord)| {
            if ord != Ordering::Greater {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(v: Vec<String>) -> usize {
    let decoders = ["[[2]]", "[[6]]"]
        .into_iter()
        .map(|s| (s.to_string(), true));

    v.into_iter()
        .filter(|l| !l.is_empty())
        .map(|s| (s, false))
        .chain(decoders)
        .map(|(l, d)| (parse(&l), d))
        .sorted_by(|(x, _), (y, _)| compare(x, y, x.root, y.root))
        .enumerate()
        .filter(|(_, (_, d))| *d)
        .map(|(i, _)| i + 1)
        .product()
}
