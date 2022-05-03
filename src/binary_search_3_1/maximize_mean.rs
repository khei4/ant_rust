fn check(x: f64, wv: &Vec<(f64, f64)>, k: usize) -> bool {
    let mut tmp = 0.0;
    let mut vals: Vec<f64> = wv.iter().map(|(w, v)| v - x * w).collect();
    vals.sort_by(|a, b| b.partial_cmp(a).expect("Nan"));
    for i in 0..k {
        tmp += vals[i];
    }
    tmp >= 0.0
}

fn main() {
    let (n, k): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut wv = vec![];
    for _ in 0..n {
        wv.push((read::<f64>(), read::<f64>()));
    }
    let (mut ok, mut ng) = (0.0, 10f64.powi(6) + 1.0);
    while ng - ok > 1e-7 {
        let m = (ok + ng) / 2.0;
        if check(m, &wv, k) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
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
