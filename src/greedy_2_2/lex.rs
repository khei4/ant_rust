fn main() {
    let n: usize = read();
    let s: Vec<char> = read::<String>().chars().collect();
    let mut s_rev = s.clone();
    s_rev.reverse();
    let (mut a, mut b) = (0, n - 1);
    'outer: while a <= b {
        let mut i = 0;
        while a + i < b - i {
            if s[a + i] < s[b - i] {
                print!("{}", s[a]);
                a += 1;
                continue 'outer;
            } else if s[a + i] > s[b - i] {
                print!("{}", s[b]);
                b -= 1;
                continue 'outer;
            }
            i += 1;
        }
        print!("{}", s[b]);
        b -= 1;
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
