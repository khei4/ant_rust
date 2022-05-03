mod Kruscal {
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
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Edge {
        pub from: usize,
        pub to: usize,
        pub cost: i64,
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Edge) -> std::cmp::Ordering {
            self.cost.cmp(&other.cost)
        }
    }

    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Edge) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }
    pub fn build(v: usize, edges: &mut Vec<Edge>) -> (Vec<Edge>, i64) {
        let mut uf = UnionFind::new(v);
        // sort ascending order
        edges.sort();
        // remove duplicated edge
        edges.dedup();
        let mut res_tree: Vec<Edge> = vec![];
        let mut res: i64 = 0;
        // till graph is connected
        for e in edges {
            if !uf.same(e.from, e.to) {
                uf.union(e.from, e.to);
                res_tree.push(*e);
                res += e.cost;
            }
        }
        (res_tree, res)
    }
}

fn main() {
    let (n, m, r): (usize, usize, usize) = (read::<usize>(), read::<usize>(), read::<usize>());
    use Kruscal::{build, Edge};
    let mut edges: Vec<Edge> = vec![];
    for _ in 0..r {
        let (u, v, c): (usize, usize, i64) = (read::<usize>(), read::<usize>(), read::<i64>());
        edges.push(Edge {
            from: u,
            to: n + v,
            cost: -c,
        });
        edges.push(Edge {
            from: n + v,
            to: u,
            cost: -c,
        });
    }
    println!("{}", 10000 * (n + m) as i64 + build(n + m, &mut edges).1);
}

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
