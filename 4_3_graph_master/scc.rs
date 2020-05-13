fn dfs(v: usize, adjl: &Vec<Vec<usize>>, used: &mut Vec<bool>, vs: &mut Vec<usize>) {
    if used[v] {
        return;
    } else {
        used[v] = true;
        for u in &adjl[v] {
            dfs(*u, adjl, used, vs);
        }
        vs.push(v);
    }
}

fn rdfs(v: usize, k: usize, radjl: &Vec<Vec<usize>>, used: &mut Vec<bool>, cmp: &mut Vec<usize>) {
    if used[v] {
        return;
    } else {
        used[v] = true;
        cmp[v] = k;
        for u in &radjl[v] {
            rdfs(*u, k, radjl, used, cmp);
        }
    }
}

fn main() {
    let (n, m): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut adjl = vec![vec![]; n];
    let mut radjl = vec![vec![]; n];
    for _ in 0..m {
        let (u, v): (usize, usize) = (read::<usize>(), read::<usize>());
        adjl[u].push(v);
        radjl[v].push(u);
    }
    let mut used = vec![false; n];
    let mut vs = vec![];
    for v in 0..n {
        if !used[v] {
            dfs(v, &adjl, &mut used, &mut vs);
        }
    }
    used = vec![false; n];
    let mut cmp = vec![0; n];
    let mut k = 0;
    for v in vs.iter().rev() {
        if !used[*v] {
            rdfs(*v, k, &radjl, &mut used, &mut cmp);
            k += 1
        }
    }
    println!("{}", k);
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
