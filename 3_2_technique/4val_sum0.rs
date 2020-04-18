fn lower_bound<T: std::cmp::Ord>(arr: &Vec<T>, b: T) -> usize {
    let (mut lt, mut ge) = (0, arr.len());
    // lt=-1にすると型強制しなきゃなのでここで確かめる
    if arr[lt] >= b {
        return 0;
    }
    while lt + 1 < ge {
        let m = (lt + ge) / 2;
        if arr[m] < b {
            lt = m;
        } else if arr[m] >= b {
            ge = m;
        }
    }
    ge
}

fn main() {
    let n = read();
    let (a, b, c, d): (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) = (
        (0..n).map(|_| read::<i64>()).collect(),
        (0..n).map(|_| read::<i64>()).collect(),
        (0..n).map(|_| read::<i64>()).collect(),
        (0..n).map(|_| read::<i64>()).collect(),
    );
    let mut former = vec![];
    let mut latter = vec![];
    for e in a {
        for e0 in &b {
            former.push(e + *e0);
        }
    }
    for e in c {
        for e0 in &d {
            latter.push(e + *e0);
        }
    }
    latter.sort();
    let mut res = 0;
    for f in former {
        if latter[lower_bound(&latter, -f)] == -f {
            res += 1;
        }
    }
    println!("{}", res);
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
