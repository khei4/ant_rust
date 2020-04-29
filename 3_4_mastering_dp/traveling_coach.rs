#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    cost: f64,
}

fn main() {
    // n... ticket number, v ... node number
    let (n, v): (usize, usize) = (read::<usize>(), read::<usize>());
    let (e, start, goal): (usize, usize, usize) =
        (read::<usize>(), read::<usize>() - 1, read::<usize>() - 1);
    let mut tickets: Vec<f64> = vec![];
    for _ in 0..n {
        tickets.push(read());
    }
    let mut adjl: Vec<Vec<Edge>> = vec![vec![]; v];
    for _ in 0..e {
        let (x, y, z): (usize, usize, f64) =
            (read::<usize>() - 1, read::<usize>() - 1, read::<f64>());
        adjl[x].push(Edge { to: y, cost: z });
        adjl[y].push(Edge { to: x, cost: z });
    }
    // [ticketの状態][たどりつくとこ] = 時間
    let s = (1 << n) - 1;
    let mut dp = vec![vec![std::f64::MAX; v]; s + 1];
    dp[s][start] = 0.0;
    let mut res = std::f64::MAX;
    // 進む前のticketの状態で全探索
    for ticket in (0..s + 1).rev() {
        for u in 0..v {
            for e in &adjl[u] {
                // i番目のチケットを使う
                for i in 0..n {
                    if (ticket >> i) & 1 == 1 {
                        dp[ticket & !(1 << i)][e.to] =
                            dp[ticket & !(1 << i)][e.to].min(dp[ticket][u] + e.cost / tickets[i]);
                    }
                }
            }
        }
        res = res.min(dp[ticket][goal]);
    }
    println!("{}", res);
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
