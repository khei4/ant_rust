#[derive(Debug)]
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
    fn sum(&self, mut i: usize) -> T {
        let mut sum: T = T::default();
        while i != 0 {
            sum += self.table[i];
            i &= i - 1;
        }
        sum
    }

    fn add(&mut self, mut i: usize, x: T) {
        if i == 0 {
            panic!("0 is given to BIT");
        }
        while (i as usize) < self.table.len() {
            self.table[i as usize] += x;
            i += i - (i & (i - 1));
        }
    }
}

fn main() {
    let (n, q): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut bit0 = BIT::<i64>::new(n);
    let mut bit1 = BIT::<i64>::new(n);
    for _ in 0..q {
        let c: usize = read();
        if c == 0 {
            let (l, r, x): (usize, usize, i64) = (read::<usize>(), read::<usize>(), read::<i64>());
            bit0.add(l, -x * (l - 1) as i64);
            bit1.add(l, x);
            bit0.add(r + 1, x * r as i64);
            bit1.add(r + 1, -x);
        } else if c == 1 {
            let (l, r): (usize, usize) = (read::<usize>(), read::<usize>());
            println!(
                "{}",
                (bit0.sum(r) + bit1.sum(r) * r as i64)
                    - (bit0.sum(l - 1) + bit1.sum(l - 1) * (l - 1) as i64)
            );
        } else {
            unreachable!();
        }
    }
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
