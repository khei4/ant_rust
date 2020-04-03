fn check(x: &Vec<usize>, d: usize, m: usize) -> bool {
    // 前から入れて最後にいれたうしのいち
    let mut cur = x[0];
    let mut rem = m - 1;
    for i in 1..x.len() {
        if x[i] - cur >= d {
            cur = x[i];
            rem -= 1;
            if rem == 0 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let (n, m): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut x = vec![];
    for i in 0..n {
        x.push(read());
    }
    x.sort();
    let (mut okd, mut ngd) = (0, 10usize.pow(9) + 1);
    while ngd - okd > 1 {
        let mid = (okd + ngd) / 2;
        if check(&x, mid, m) {
            okd = mid;
        } else {
            ngd = mid;
        }
    }

    println!("{}", okd);
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
