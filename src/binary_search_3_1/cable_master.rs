fn check(l: &Vec<f64>, v: f64, k: f64) -> bool {
    let mut tmp = 0.0;
    for i in 0..l.len() {
        //個数なのでfloor
        tmp += (l[i] / v).floor();
    }
    if tmp >= k {
        true
    } else {
        false
    }
}

fn main() {
    let (n, k): (usize, f64) = (read::<usize>(), read::<f64>());
    let mut l = vec![];
    for _ in 0..n {
        l.push(read());
    }
    let (mut ok, mut ng) = (0.0, 100001.0);
    while ng - ok > 1e-6 {
        let m = (ok + ng) / 2.0;
        if check(&l, m, k) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{:.2}", ok);
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
