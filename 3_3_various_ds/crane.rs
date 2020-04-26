#[derive(Debug)]
struct SegTree {
    vx: Vec<f64>,
    vy: Vec<f64>,
    ang: Vec<f64>,
}

impl SegTree {
    fn new(size: usize) -> SegTree {
        SegTree {
            vx: vec![0.0; size],
            vy: vec![0.0; size],
            ang: vec![0.0; size],
        }
    }
    fn init(&mut self, k: usize, l: usize, r: usize, length: &Vec<f64>) {
        self.ang[k] = 0.0;
        self.vx[k] = 0.0;
        // 葉
        if r - l == 1 {
            self.vy[k] = length[l];
        } else {
            let (chl, chr) = ((k << 1) + 1, (k << 1) + 2);
            self.init(chl, l, (l + r) / 2, length);
            self.init(chr, (l + r) / 2, r, length);
            // 直結
            self.vy[k] = self.vy[chl] + self.vy[chr];
        }
    }
    // sが対象, a は値, vは現在, l,rはvの対応する区間
    fn change(&mut self, s: usize, a: f64, v: usize, l: usize, r: usize) {
        if s <= l {
            return;
        } else if s < r {
            let (chl, chr) = (v << 1 + 1, v << 1 + 2);
            let m = (l + r) / 2;
            self.change(s, a, chl, l, m);
            self.change(s, a, chr, m, r);
            if s <= m {
                self.ang[v] += a;
            }
            let s = self.ang[v].sin();
            let c = self.ang[v].cos();
            self.vx[v] = self.vx[chl] + (c * self.vx[chr] - s * self.vy[chr]);
            self.vy[v] = self.vy[chl] + (s * self.vy[chr] + c * self.vy[chr]);
        }
    }
}

fn main() {
    let n: usize = read();
    let c: usize = read();
    let l: Vec<f64> = (0..n).map(|_| read::<f64>()).collect();
    let s: Vec<f64> = (0..c).map(|_| read::<f64>()).collect();
    let a: Vec<f64> = (0..c).map(|_| read::<f64>()).collect();
    let mut sg = SegTree::new(10usize.pow(1));
    sg.init(0, 0, n, &l);
    println!("{:?}", sg);
    sg.change()
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
