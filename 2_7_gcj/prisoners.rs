fn main() {
    let (p, q): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut a: Vec<usize> = vec![0];
    for _ in 0..q {
        a.push(read());
    }
    a.sort();
    a.push(p + 1);
    //dp[左端][右端]=その部屋(インデックス)の開放すべきやつらを開放するのに必要な最小
    let mut dp = vec![vec![0; q + 2]; q + 2];

    // みる部屋の幅
    for w in 2..q + 2 {
        //左端
        for l in 0..q {
            // 右端(閉)
            let r = l + w;
            if r > q + 1 {
                break;
            }
            let mut t = std::usize::MAX;
            // jは,(l,r)を最初に開ける位置
            for j in l + 1..r {
                t = min(t, dp[l][j] + dp[j][r]);
            }
            dp[l][r] = a[r] - a[l] - 2 + t;
        }
    }
    println!("{}", dp[0][q + 1]);
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
