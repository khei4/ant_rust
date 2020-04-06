#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    cost: usize,
}
fn rec(adjl: &Vec<Vec<Edge>>, dp: &mut Vec<Vec<usize>>, s: usize, v: usize) -> usize {
    // println!("{:05b}, {}", s, v);
    let n = adjl.len();
    // ベースケース(すべてのbitが1で,帰ってきた)
    if s == (1 << n) - 1 && v == 0 {
        dp[s][v] = 0;
        return dp[s][v];
    }
    let mut res = std::usize::MAX / 2;
    for e in &adjl[v] {
        // 未探索 なら次のインデックスビット目が0
        if (s >> e.to) & 1 == 0 {
            res = min(res, rec(adjl, dp, s | 1usize << e.to, e.to) + e.cost);
        }
    }
    // println!("{}", res);
    dp[s][v] = res;
    return dp[s][v];
}

fn main() {
    let (v, e): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut adjl = vec![vec![]; v];
    for _ in 0..e {
        let (u, v, c): (usize, usize, usize) = (read::<usize>(), read::<usize>(), read::<usize>());
        adjl[u].push(Edge { to: v, cost: c });
    }
    let mut dp = vec![vec![std::usize::MAX; v]; 1 << v];
    println!("{}", rec(&adjl, &mut dp, 0, 0));
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
