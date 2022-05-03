fn lower_bound(arr: &Vec<(usize, usize)>, b: usize) -> usize {
    let (mut lt, mut ge) = (0, arr.len());
    // lt=-1にすると型強制しなきゃなのでここで確かめる
    if arr[lt].0 >= b {
        return 0;
    }
    while lt + 1 < ge {
        let m = (lt + ge) / 2;
        if arr[m].0 < b {
            lt = m;
        } else if arr[m].0 >= b {
            ge = m;
        }
    }
    ge
}
// 以下な最大要素をかえしたい
fn le(b: usize, arr: &Vec<(usize, usize)>) -> usize {
    let (mut le, mut gt) = (0, arr.len());
    // lt=-1にすると型強制しなきゃなのでここで確かめる
    if arr[le].0 > b {
        return 0;
    }
    while le + 1 < gt {
        let m = (le + gt) / 2;
        if arr[m].0 <= b {
            le = m;
        } else if arr[m].0 >= b {
            gt = m;
        }
    }
    arr[le].1
}

fn main() {
    let n: usize = read();
    let mut w_a: Vec<usize> = (0..n).map(|_| read::<usize>()).collect();
    let mut v_a: Vec<usize> = (0..n).map(|_| read::<usize>()).collect();
    let capa: usize = read();
    let half_a = n / 2;
    let half_b = n - n / 2;
    let w_b = w_a.split_off(half_a);
    let v_b = v_a.split_off(half_a);
    let mut a: Vec<(usize, usize)> = vec![];
    for i in 0..1 << half_a {
        let (mut w_sum, mut v_sum) = (0, 0);
        for j in 0..half_a {
            if 1 << j & i != 0 {
                w_sum += w_a[j];
                v_sum += v_a[j];
            }
        }
        if w_sum > capa {
            continue;
        } else {
            a.push((w_sum, v_sum));
        }
    }
    let mut b: Vec<(usize, usize)> = vec![];
    for i in 0..1 << half_b {
        let (mut w_sum, mut v_sum) = (0, 0);
        for j in 0..half_b {
            if 1 << j & i != 0 {
                w_sum += w_b[j];
                v_sum += v_b[j];
            }
        }
        if w_sum > capa {
            continue;
        } else {
            b.push((w_sum, v_sum));
        }
    }
    // 重さの昇順
    a.sort_by(|x, y| x.0.cmp(&y.0));
    b.sort_by(|x, y| x.0.cmp(&y.0));
    // 価値の昇順
    a.sort_by(|x, y| x.1.cmp(&y.1));
    b.sort_by(|x, y| x.1.cmp(&y.1));

    // それぞれから一つしか選べないので, あるセットより価値が同じか低くくて重いのはいらない
    // これをするとx.weight < y.weight => x.value < y.valueになる
    let (mut sel_a, mut sel_b) = (vec![], vec![]);
    let mut prev_v = 0;
    for e in a {
        // 0もとりたいので下駄
        if prev_v < e.1 + 1 {
            prev_v = e.1 + 1;
            sel_a.push(e);
        }
    }
    prev_v = 0;
    for e in b {
        if prev_v < e.1 + 1 {
            prev_v = e.1 + 1;
            sel_b.push(e);
        }
    }
    // にぶたんを重さに対してして,キャパの限界まで入れれば良い
    let mut res = 0;
    for a in sel_a {
        match capa.checked_sub(a.0) {
            Some(rem) => {
                res = std::cmp::max(res, a.1 + le(rem, &sel_b));
            }
            None => (),
        }
        // let cap_rem = capa - a.0;
        // let lb = lower_bound(&sel_b, cap_rem);
        // if sel_b[lb].0 == cap_rem {
        //     res = std::cmp::max(res, a.1 + sel_b[lb].1)
        // } else {
        //     // 0があるので大丈夫
        //     res = std::cmp::max(res, a.1 + sel_b[lb - 1].1)
        // }
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
