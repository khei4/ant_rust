fn main() {
    let n = read();
    println!("{}", n);
    let mut x: Vec<isize> = vec![];
    for _ in 0..n {
        x.push(read());
    }
    let mut y: Vec<isize> = vec![];
    for _ in 0..n {
        y.push(read());
    }
    x.sort_by(|x, y| y.cmp(x));
    y.sort();
    let mut ans = 0;
    for i in 0..n {
        ans += x[i] * y[i];
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
