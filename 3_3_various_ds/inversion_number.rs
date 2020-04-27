struct BIT<T> {
    table: Vec<T>,
}
impl<T: std::default::Default + Clone + Copy + std::ops::AddAssign> BIT<T> {
    fn new(n: usize) -> BIT<T> {
        BIT {
            table: vec![T::default(); n + 1],
        }
    }
    // 1..i番目の要素までの和を求める
    fn sum(&self, mut i: i64) -> T {
        let mut sum: T = T::default();
        while i != 0 {
            sum += self.table[i as usize];
            i -= i & -i;
        }
        sum
    }

    fn add(&mut self, mut i: i64, x: T) {
        while (i as usize) < self.table.len() {
            self.table[i as usize] += x;
            i += i & -i;
        }
    }
}

fn main() {
    let n: usize = read();
    let mut bit = BIT::<usize>::new(n);
    let mut ans = 0;
    for i in 0..n {
        let v: i64 = read();
        ans += i - bit.sum(v);
        bit.add(v, 1);
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
