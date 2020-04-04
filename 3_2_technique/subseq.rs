fn main() {
    let (n, s): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut a: Vec<usize> = vec![];
    for _ in 0..n {
        a.push(read());
    }
    let (mut l, mut r) = (0, 0);
    let mut sum = 0;
    let mut ans = std::usize::MAX;
    loop {
        while sum < s && r < n {
            sum += a[r];
            r += 1;
        }
        if sum < s {
            break;
        }
        ans = min(r - l, ans);
        sum -= a[l];
        l += 1;
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
