fn main() {
    let (n, l, d): (usize, usize, usize) = (read::<usize>(), read::<usize>(), read::<usize>());
    let mut edges: Vec<Edge> = vec![];
    for i in 0..n - 1 {
        edges.push(Edge {
            from: i,
            to: i + 1,
            cost: 0,
        });
    }
    for _ in 0..l {
        let (s, t, w): (usize, usize, i64) =
            (read::<usize>() - 1, read::<usize>() - 1, read::<i64>());
        edges.push(Edge {
            from: s,
            to: t,
            cost: w,
        });
    }
    for _ in 0..d {
        let (s, t, w): (usize, usize, i64) =
            (read::<usize>() - 1, read::<usize>() - 1, read::<i64>());
        edges.push(Edge {
            from: s,
            to: t,
            cost: -w,
        });
    }
    match shortest_paths(n, &edges, 0) {
        Some(dist) => {
            if dist[n - 1] == std::i64::MAX {
                println!("-2",);
            } else {
                println!("{}", dist[n - 1]);
            }
        }
        None => {
            println!("-1");
        }
    }
}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

// n: vertex number
fn shortest_paths(v_num: usize, edges: &Vec<Edge>, start: usize) -> Option<Vec<i64>> {
    let mut dist = vec![std::i64::MAX; v_num];
    dist[start] = 0;
    let mut updated = true;
    let mut cnt = 0;
    while updated && cnt <= v_num {
        updated = false;
        for e in edges {
            if dist[e.from] != std::i64::MAX && dist[e.to] > dist[e.from] + e.cost {
                updated = true;
                dist[e.to] = dist[e.from] + e.cost;
            }
        }
        cnt += 1;
    }
    if cnt == v_num + 1 {
        None
    } else {
        Some(dist)
    }
}
// =========
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::process::exit;

#[allow(dead_code)]
const MOD: usize = 1000000007;

fn read<T: std::str::FromStr>() -> T {
    use std::io::Read;
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

// =========
