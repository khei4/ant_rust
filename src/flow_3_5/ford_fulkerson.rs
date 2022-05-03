/*
Ford-Fulkerson
*/
#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    cap: i64,
    rev_e: usize, // この辺の逆辺のインデックス
}

fn add_edge(from: usize, to: usize, cap: i64, adjl: &mut Vec<Vec<Edge>>) {
    let rev_e = adjl[to].len();
    adjl[from].push(Edge { to, cap, rev_e });
    let rev_e = adjl[from].len() - 1;
    adjl[to].push(Edge {
        to: from,
        cap,
        rev_e,
    });
}

fn dfs(v: usize, t: usize, f: i64, used: &mut Vec<bool>, adjl: &mut Vec<Vec<Edge>>) -> i64 {
    if v == t {
        // ゴールなら,そのときのフロー
        f
    } else {
        for i in 0..adjl[v].len() {
            let mut e = adjl[v][i];
            if !used[e.to] && e.cap > 0 {
                // 辺のキャパシティか流量
                let d = dfs(e.to, t, min(f, e.cap), used, adjl);
                // 逆辺を張る
                if d > 0 {
                    e.cap -= d;
                    adjl[e.to][e.rev_e].cap += d;
                    return d;
                }
            }
        }
        // s, t が非連結なときなど
        0
    }
}
fn max_flow(s: usize, t: usize, used: &mut Vec<bool>, adjl: &mut Vec<Vec<Edge>>) -> i64 {
    let mut flow = 0;
    loop {
        let f = dfs(s, t, std::i64::MAX, used, adjl);

        if f == 0 {
            return flow;
        }
        flow += f;
    }
}
fn main() {}

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
