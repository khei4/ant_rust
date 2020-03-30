fn main() {
    let (n, r): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut adjl = vec![vec![]; n];
    for _ in 0..r {
        let (u, v, c): (usize, usize, usize) =
            (read::<usize>() - 1, read::<usize>() - 1, read::<usize>());
        adjl[u].push(Edge { node: v, cost: c });
        adjl[v].push(Edge { node: u, cost: c });
    }
    println!("{}", shortest_second_path(&adjl, 0)[n - 1]);
}

// =========
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_second_path(adjl: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..adjl.len()).map(|_| std::usize::MAX).collect();
    let mut dist2: Vec<_> = (0..adjl.len()).map(|_| std::usize::MAX).collect();

    let mut heap = std::collections::BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist2[position] {
            continue;
        }

        for edge in &adjl[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            // 最短だった場合
            if next.cost < dist[next.position] {
                heap.push(next);
                // 降格、二番目も入れる
                dist2[next.position] = dist[next.position];
                heap.push(State {
                    cost: dist2[next.position],
                    position: edge.node,
                });
                dist[next.position] = next.cost;
            }
            // 二番目だった場合
            else if next.cost < dist2[next.position] {
                heap.push(next);
                dist2[next.position] = next.cost;
            }
        }
    }
    dist2
}
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
