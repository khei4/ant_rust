fn lower_bound<T: std::cmp::Ord>(arr: &Vec<T>, b: T) -> usize {
    let (mut ok, mut ng) = (0, arr.len());
    if arr[ok] > b {
        return 0;
    }
    while ok + 1 < ng {
        let m = (ok + ng) / 2;
        if arr[m] < b {
            ok = m;
        } else if arr[m] > b {
            ng = m;
        } else {
            return m;
        }
    }
    if arr[ok] == b {
        ok
    } else {
        ok + 1
    }
}

fn main() {
    let n = read();
    let mut v: Vec<usize> = vec![];
    for _ in 0..n {
        v.push(read());
    }
    let mut dp = vec![std::usize::MAX; n];
    let mut ans = 0;
    for i in 0..n {
        let b = lower_bound(&dp, v[i]);
        dp[b] = v[i];
        ans = max(ans, b + 1);
    }
    println!("{}", ans);
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
