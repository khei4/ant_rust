fn main() {
    let (n, k): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut uf = UnionFind::new(3 * n);
    let mut ans = 0;
    for _ in 0..k {
        let (t, x, y): (usize, usize, usize) = (read::<usize>(), read::<usize>(), read::<usize>());
        if x > n || y > n {
            ans += 1;
            continue;
        }
        if t == 1 {
            if uf.same(x + n, y) || uf.same(x + 2 * n, y) {
                ans += 1;
                continue;
            } else {
                for i in 0..2 {
                    uf.union(x + i * n, y + i * n);
                }
            }
        } else if t == 2 {
            if uf.same(x, y + 2 * n) || uf.same(x, y) {
                ans += 1;
                continue;
            } else {
                for i in 0..2 {
                    uf.union(x + i * n, y + ((i + 1) % 2) * n);
                }
            }
        } else {
            unreachable!();
        }
    }
    println!("{}", ans);
}

//
/*
中身はi64だけど外はusizeにしてみる
*/

// ============
#[derive(Debug)]
struct UnionFind {
    // size= 親なら負のサイズ、子なら親
    // number= 集合の数
    table: Vec<i64>,
    number: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let table = vec![-1; n];
        UnionFind {
            table: table,
            number: n,
        }
    }
}
impl UnionFind {
    fn root(&mut self, x: usize) -> usize {
        let par = self.table[x];
        if par < 0 {
            x
        } else {
            let tmp = self.root(par as usize);
            self.table[x] = tmp as i64;
            tmp
        }
    }
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    fn union(&mut self, a: usize, b: usize) -> () {
        let a_root = self.root(a);
        let b_root = self.root(b);

        if a_root == b_root {
            return ();
        }
        // 負なので小さい法が大きい. 大きい方につける
        if self.table[a_root] > self.table[b_root] {
            self.table[b_root] += self.table[a_root];
            self.table[a_root] = b_root as i64;
        } else {
            self.table[a_root] += self.table[b_root];
            self.table[b_root] = a_root as i64;
        }
        self.number -= 1;
    }
    // 親のサイズを返す
    fn size(&mut self, x: usize) -> usize {
        let ri = self.root(x);
        -self.table[ri] as usize
    }
    fn count(&self) -> usize {
        self.number
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
