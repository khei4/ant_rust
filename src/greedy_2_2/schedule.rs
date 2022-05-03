fn main() {
    let n = read();
    let mut job: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        job.push((read(), 0));
    }
    for i in 0..n {
        job[i].1 = read();
    }
    let mut ans = 0;
    let mut tmpend = 0;
    for i in 0..n {
        if tmpend < job[i].0 {
            ans += 1;
            tmpend = job[i].1;
        }
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
