//座圧パートだけ..
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

fn compres(starts: &mut Vec<usize>, ends: &mut Vec<usize>, lim: usize) -> usize {
    let mut needed = vec![];

    for i in 0..starts.len() {
        for d in [-1, 0, 1].iter() {
            if (starts[i] != 0 || *d != -1) && (starts[i] != lim - 1 || *d == 1) {
                needed.push((starts[i] as i64 + d) as usize);
            }
            if (ends[i] != 0 || *d != -1) && (ends[i] != lim - 1 || *d == 1) {
                needed.push((ends[i] as i64 + d) as usize);
            }
        }
    }
    needed.sort();
    needed.dedup();
    for i in 0..starts.len() {
        starts[i] = lower_bound(&needed, starts[i]);
        ends[i] = lower_bound(&needed, ends[i]);
    }
    needed.len()
}

fn main() {
    let (w, h, n): (usize, usize, usize) = (read::<usize>(), read::<usize>(), read::<usize>());
    let (mut xs, mut xe): (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    let (mut ys, mut ye): (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    let mut xs: Vec<usize> = (0..n).map(|_| read::<usize>() - 1).collect();
    let mut xe: Vec<usize> = (0..n).map(|_| read::<usize>() - 1).collect();
    let mut ys: Vec<usize> = (0..n).map(|_| read::<usize>() - 1).collect();
    let mut ye: Vec<usize> = (0..n).map(|_| read::<usize>() - 1).collect();

    let w = compres(&mut xs, &mut xe, w);
    let h = compres(&mut ys, &mut ye, h);
    // 端点とその前後だけを取る.
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
