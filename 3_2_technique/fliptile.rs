fn main() {
    let (m, n): (usize, usize) = (read::<usize>(), read::<usize>());
    let mut origin_map: Vec<Vec<usize>> = vec![];
    for _ in 0..m {
        let mut row = vec![];
        for _ in 0..n {
            row.push(read());
        }
        origin_map.push(row);
    }
    let mut flip_times = vec![vec![0; n]; m];
    let mut colors = vec![vec![0; n]; m];
    let mut flip_time = std::usize::MAX;
    let mut res: Vec<Vec<usize>> = vec![];

    for i in 0..(1 << n) {
        let mut tmpres = 0;
        // １行目決め
        // println!("{:04b}", i);
        for j in 0..n {
            if 1 << n - j - 1 & i != 0 {
                flip_times[0][j] = 1;
                tmpres += 1;
            } else {
                flip_times[0][j] = 0;
            }
        }
        // 一行目の色が決まる
        // println!("flip times{:?}", flip_times[0]);
        for j in 0..n {
            match (j.checked_sub(1), j, j + 1) {
                (None, j, jf) if jf < n => {
                    colors[0][j] = (origin_map[0][j] + flip_times[0][j] + flip_times[0][jf]) % 2
                }
                (Some(jb), j, _jf) if _jf >= n => {
                    colors[0][j] = (origin_map[0][j] + flip_times[0][jb] + flip_times[0][j]) % 2
                }
                (Some(jb), j, jf) => {
                    colors[0][j] = (origin_map[0][j]
                        + flip_times[0][jb]
                        + flip_times[0][j]
                        + flip_times[0][jf])
                        % 2
                }
                (_, _, _) => unreachable!(),
            }
        }
        // println!("origin{:?}", origin_map[0]);
        // println!("changed{:?}", colors[0]);

        // 下の行目(j)を埋めていく
        for j in 1..m {
            // 反転させるマス決め
            for k in 0..n {
                if colors[j - 1][k] == 1 {
                    flip_times[j][k] = 1;
                    tmpres += 1;
                } else {
                    flip_times[j][k] = 0;
                }
            }
            // 元,上,自分,左右を見て色決め
            for k in 0..n {
                match (k.checked_sub(1), k, k + 1) {
                    (None, k, kf) if kf < n => {
                        colors[j][k] = (origin_map[j][k]
                            + flip_times[j - 1][k]
                            + flip_times[j][k]
                            + flip_times[j][kf])
                            % 2
                    }
                    (Some(kb), k, _kf) if _kf >= n => {
                        colors[j][k] = (origin_map[j][k]
                            + flip_times[j - 1][k]
                            + flip_times[j][k]
                            + flip_times[j][kb])
                            % 2
                    }
                    (Some(kb), k, kf) => {
                        colors[j][k] = (origin_map[j][k]
                            + flip_times[j - 1][k]
                            + flip_times[j][k]
                            + flip_times[j][kb]
                            + flip_times[j][kf])
                            % 2
                    }
                    (_, _, _) => unreachable!(),
                }
            }
        }
        // 最後の行がall 0か
        // println!("last_row{:?}", flip_times[m - 1]);
        let c = flip_times[m - 1].iter().fold(0, |s, e| s + e);
        // println!("sum{:?}", c);
        if c != 0 {
            continue;
        } else {
            if tmpres < flip_time {
                flip_time = tmpres;
                res = flip_times.clone();
            }
        }
    }
    if res.len() == 0 {
        println!("IMPOSSIBLE");
    } else {
        for row in res {
            for e in row {
                print!("{} ", e);
            }
            println!("");
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
const MOD: isize = 1000000007;

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
