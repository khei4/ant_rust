#[derive(Debug, PartialEq, Eq)]
struct MinInt {
    v: usize,
}
impl std::cmp::PartialOrd for MinInt {
    fn partial_cmp(&self, other: &MinInt) -> Option<std::cmp::Ordering> {
        other.v.partial_cmp(&self.v)
    }
}
impl std::cmp::Ord for MinInt {
    fn cmp(&self, other: &MinInt) -> std::cmp::Ordering {
        other.v.cmp(&self.v)
    }
}

fn main() {
    let n: usize = read();
    let mut heap: BinaryHeap<MinInt> = BinaryHeap::new();
    for _ in 0..n {
        heap.push(MinInt { v: read() });
    }
    let mut ans = 0;
    while heap.len() != 1 {
        let (l, r) = (
            heap.pop().expect("something wrong"),
            heap.pop().expect("something wrong"),
        );
        let s = l.v + r.v;
        ans += s;
        heap.push(MinInt { v: s });
    }
    println!("{}", ans);
}

// =========
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::BinaryHeap;
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
