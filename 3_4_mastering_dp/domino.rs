const N: usize = 3;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Matrix<T>
where
    T: Copy + Clone,
{
    val: [[T; N]; N],
}

// Debug Display
impl<T> std::fmt::Debug for Matrix<T>
where
    T: std::fmt::Debug + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..N {
            res = format!("{}{:?}\n", res, self.val[i]);
        }
        write!(f, "{}", res)
    }
}

// constructors
#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: From<i32> + Copy + Clone,
{
    fn i() -> Self {
        let mut val: [[T; N]; N] = [[T::from(0i32); N]; N];
        for i in 0..N {
            val[i][i] = T::from(1i32);
        }
        Matrix { val: val }
    }
    fn zero() -> Self {
        let val: [[T; N]; N] = [[T::from(0i32); N]; N];
        Matrix { val: val }
    }
    // generate permutation matrix
    fn permutation(pi: usize, pj: usize) -> Self {
        let mut val: [[T; N]; N] = [[T::from(0i32); N]; N];
        for i in 0..N {
            if i == pi {
                val[i][pj] = T::from(1i32);
            } else if i == pj {
                val[i][pi] = T::from(1i32);
            } else {
                val[i][i] = T::from(1i32);
            }
        }
        Matrix { val: val }
    }
}

impl<T: Copy + Clone> From<[[T; N]; N]> for Matrix<T> {
    fn from(m: [[T; N]; N]) -> Self {
        Matrix { val: m }
    }
}

fn main() {
    let (n, m): (usize, usize) = (read::<usize>(), read::<usize>());
    // 1 -> black, 0 -> white
    let mut color = vec![];
    for _ in 0..n {
        let s: Vec<usize> = read::<String>()
            .chars()
            .map(|e| if e == 'x' { 1 } else { 0 })
            .collect();
        color.push(s)
    }
    println!("{:?}", color);
    let mut cur = vec![0; 1 << m];
    let mut next = vec![0; 1 << m];
    cur[0] = 1;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            for used in 0..1 << m {
                if used >> j & 1 == 1 || color[i][j] == 1 {
                    next[used] = cur[used & !(1 << j)];
                } else {
                    let mut r = 0;
                    // よこ？
                    if j + 1 < m && used >> (j + 1) & 1 != 1 && color[i][j + 1] != 1 {
                        r += cur[used | 1 << (j + 1)];
                    }
                    // たて
                    if i + 1 < n && color[i + 1][j] != 1 {
                        r += cur[used | 1 << j];
                    }
                    next[used] = r % MOD;
                }
            }
            std::mem::swap(&mut cur, &mut next);
        }
    }
    println!("{}", cur[0]);
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
