fn main() {
    let (n, m): (usize, usize) = (read::<usize>(), read::<usize>());
    let md: usize = read();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0][0] = 1;
    for i in 1..=m {
        for j in 0..=n {
            if let Some(rm) = j.checked_sub(i) {
                // 0があるケースと、全要素から-1をした分割数
                dp[i][j] = (dp[i - 1][j] + dp[i][rm]) % md;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    println!("{}", dp[m][n]);
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
