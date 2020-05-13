#[derive(Debug, Clone)]
struct Matrix {
    val: Vec<Vec<i64>>,
}

impl Matrix {
    fn mul(&self, rhs: &Matrix) -> Self {
        assert_eq!(self.val[0].len(), rhs.val.len());
        let n = self.val[0].len();
        let mut res = vec![vec![0; rhs.val[0].len()]; self.val.len()];
        for i in 0..self.val.len() {
            for j in 0..n {
                for k in 0..rhs.val[0].len() {
                    res[i][k] = res[i][k] + (self.val[i][j] * rhs.val[j][k]);
                }
            }
        }
        Matrix { val: res }
    }
    fn pow(&mut self, mut n: u32) -> Matrix {
        let mut base = self.clone();
        let mut res: Matrix = Matrix {
            val: vec![vec![0; self.val.len()]; self.val.len()],
        };
        for i in 0..self.val.len() {
            res.val[i][i] = 1;
        }
        while n > 0 {
            if n & 1 == 1 {
                res = res.mul(&base);
            }
            base = base.mul(&base);
            n >>= 1;
        }
        res
    }
}

fn main() {
    let n: u32 = read();
    let mut a = Matrix {
        val: vec![vec![0, 1], vec![1, 1]],
    };
    let f = a.pow(n);
    println!("{:?}", f);
    println!("{}", f.val[1][0]);
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
