fn main() {
    let (n, dist, mut curfuel): (usize, usize, usize) =
        (read::<usize>(), read::<usize>(), read::<usize>());
    let mut pc: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        pc.push((read(), 0));
    }
    for i in 0..n {
        pc[i].1 = read();
    }
    pc.push((dist, 0));
    let mut heap = BinaryHeap::new();
    let mut cur = 0;
    let mut i = 0;
    let mut ans = 0;
    while i <= n {
        // たどり着けるかどうか
        if let Some(rem) = curfuel.checked_sub(pc[i].0 - cur) {
            cur = pc[i].0;
            heap.push(pc[i].1);
            curfuel = rem;
        } else {
            // 補給
            if let Some(c) = heap.pop() {
                ans += 1;
                curfuel += c;
                continue;
            } else {
                println!("{}", -1);
                exit(0);
            }
        }
        i += 1;
    }
    println!("{}", ans);
}

// =========
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};
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
