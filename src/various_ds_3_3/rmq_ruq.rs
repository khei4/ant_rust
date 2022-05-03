// n は葉の数
#[derive(Debug)]
struct LazySegTree {
    leaf_num: usize,
    data: Vec<usize>,
    lazy: Vec<usize>,
}

impl LazySegTree {
    // vectorから遅延セグ木をつくる
    fn new(v: Vec<usize>) -> LazySegTree {
        let leaf_num = v
            .len()
            .checked_next_power_of_two()
            .expect("seg tree size overflow");
        let n = leaf_num * 2;
        let mut data = vec![2usize.pow(31) - 1; n - 1];
        let lazy = vec![2usize.pow(31) - 1; n - 1];
        for i in 0..v.len() {
            data[i + leaf_num - 1] = v[i];
        }
        // 更新どころ
        for i in (0..leaf_num - 1).rev() {
            data[i] = std::cmp::min(data[i * 2 + 1], data[i * 2 + 2])
        }
        LazySegTree {
            leaf_num,
            data,
            lazy,
        }
    }
    // k番目のnode, [l,r)の更新をする
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 2usize.pow(31) - 1 {
            // self.data[k] = std::cmp::min(self.data[k], self.lazy[k]);
            self.data[k] = self.lazy[k];
            // 子がいる
            if r - l > 1 {
                // self.lazy[2 * k + 1] = std::cmp::min(self.lazy[2 * k + 1], self.lazy[k]);
                // self.lazy[2 * k + 2] = std::cmp::min(self.lazy[2 * k + 2], self.lazy[k]);
                self.lazy[2 * k + 1] = self.lazy[k];
                self.lazy[2 * k + 2] = self.lazy[k];
            }
            self.lazy[k] = 2usize.pow(31) - 1;
        }
    }
    // [a, b)の区間をxに変える, k, l, rは現在のnodeの情報
    fn update(
        &mut self,
        a: usize,
        b: usize,
        x: usize,
        k: Option<usize>,
        l: Option<usize>,
        r: Option<usize>,
    ) {
        // default引数のつもり
        let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.leaf_num));
        self.eval(k, l, r);
        // 範囲外
        // 完全被覆
        // 部分被覆
        if b <= l || r <= a {
            return;
        } else if a <= l && r <= b {
            self.lazy[k] = x;
            self.eval(k, l, r);
        } else {
            let (lch, rch, m) = (2 * k + 1, 2 * k + 2, (l + r) / 2);
            self.update(a, b, x, Some(lch), Some(l), Some(m));
            self.update(a, b, x, Some(rch), Some(m), Some(r));
            self.data[k] = std::cmp::min(self.data[lch], self.data[rch]);
        }
    }
    // [a,b)区間の最小値を計算
    fn get(
        &mut self,
        a: usize,
        b: usize,
        k: Option<usize>,
        l: Option<usize>,
        r: Option<usize>,
    ) -> usize {
        let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.leaf_num));
        // 範囲外なら単位元
        if b <= l || r <= a {
            return 2usize.pow(31) - 1;
        }

        self.eval(k, l, r);
        // 囲まれてるならその値
        if a <= l && r <= b {
            self.data[k]
        } else {
            let (lch, rch, m) = (2 * k + 1, 2 * k + 2, (l + r) / 2);
            std::cmp::min(
                self.get(a, b, Some(lch), Some(l), Some(m)),
                self.get(a, b, Some(rch), Some(m), Some(r)),
            )
        }
    }
}

fn main() {
    let (n, q): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut lst = LazySegTree::new(vec![2usize.pow(31) - 1; n]);
    for _ in 0..q {
        let c: usize = read();
        if c == 0 {
            lst.update(
                read::<usize>(),
                read::<usize>() + 1,
                read::<usize>(),
                None,
                None,
                None,
            );
        } else if c == 1 {
            println!(
                "{}",
                lst.get(read::<usize>(), read::<usize>() + 1, None, None, None,)
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
