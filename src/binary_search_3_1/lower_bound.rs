fn lower_bound<T: std::cmp::Ord>(arr: &Vec<T>, b: T) -> usize {
    let (mut lt, mut ge) = (0, arr.len());
    // lt=-1にすると型強制しなきゃなのでここで確かめる
    if arr[lt] >= b {
        return 0;
    }
    while lt + 1 < ge {
        let m = (lt + ge) / 2;
        if arr[m] < b {
            lt = m;
        } else if arr[m] >= b {
            ge = m;
        }
    }
    ge
}
fn main() {
    let n = read();
    let mut a: Vec<usize> = vec![];
    for _ in 0..n {
        a.push(read());
    }
    println!("{}", lower_bound(&a, read()));
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
