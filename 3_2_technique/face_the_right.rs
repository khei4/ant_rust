// 順序不問, 端からでよいので
fn flip(k: usize, s: &Vec<char>) -> Option<usize> {
    // それぞれの
    let mut flip_times = vec![0; s.len()];
    // iが反転する回数
    let mut sum = 0;
    let mut res = 0;
    for i in 0..s.len() - k + 1 {
        if (s[i] == 'B' && sum % 2 == 0) || (s[i] == 'F' && sum % 2 == 1) {
            flip_times[i] = 1;
            res += 1;
        }
        sum += flip_times[i];
        // 前の反転の場所から抜ける
        match i.checked_sub(k - 1) {
            Some(j) => {
                sum -= flip_times[j];
            }
            None => (),
        }
    }
    // 残りが前向きか
    for i in s.len() - k + 1..s.len() {
        if (s[i] == 'B' && sum % 2 == 0) || (s[i] == 'F' && sum % 2 == 1) {
            return None;
        }
        sum += flip_times[i];
        // 前の反転の場所から抜ける
        match i.checked_sub(k - 1) {
            Some(j) => {
                sum -= flip_times[j];
            }
            None => (),
        }
    }
    Some(res)
}

fn main() {
    let n: usize = read();
    let s: Vec<char> = read::<String>().chars().collect();
    // println!("{:?}", (n, s));
    // 最大回数は, 幅１
    let (mut ans_k, mut ans_m) = (1, n);
    // それぞれで, 何回でそろえられるか,
    for k in 1..n + 1 {
        match flip(k, &s) {
            Some(m) => {
                if ans_m > m {
                    ans_m = m;
                    ans_k = k;
                }
            }
            None => continue,
        }
    }
    println!("{}", ans_k);
    println!("{}", ans_m);
}

// =========

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
