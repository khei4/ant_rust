fn main() {
    let n = read();
    let mut a = vec![];
    let mut all_k = HashSet::new();
    for _ in 0..n {
        let v: usize = read();
        a.push(v);
        all_k.insert(v);
    }
    let k = all_k.len();

    let mut cur_k: HashMap<usize, usize> = HashMap::new();
    let (mut s, mut t) = (0, 0);
    let mut ans = std::usize::MAX;
    loop {
        while cur_k.len() != k && t < n {
            match cur_k.get_mut(&a[t]) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    cur_k.insert(a[t], 1);
                }
            }
            t += 1;
        }
        if cur_k.len() != k {
            break;
        } else {
            ans = min(t - s, ans);
            cur_k.remove(&a[s]);
            s += 1;
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
