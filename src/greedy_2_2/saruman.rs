fn main() {
    let n: usize = read();
    let r: usize = read();

    let mut p = vec![];
    for _ in 0..n {
        p.push(read());
    }
    // ねんため
    p.sort();
    p.push(std::usize::MAX);
    // 左端
    let mut c = 0;
    let mut ans = 0;
    while c < n - 1 {
        println!("{}", c);
        let range = p[c] + r;
        for i in c + 1..n {
            if p[i] > range {
                ans += 1;
                c = i;
                break;
            }
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
