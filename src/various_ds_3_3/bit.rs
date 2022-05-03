#[derive(Debug)]
struct BIT<T> {
    table: Vec<T>,
}
impl<T: std::default::Default + Clone + Copy + std::ops::AddAssign> BIT<T> {
    fn new(n: usize) -> BIT<T> {
        BIT {
            table: vec![T::default(); n + 1],
        }
    }
    // 1..i番目の要素までの和を求める
    fn sum(&self, mut i: usize) -> T {
        let mut sum: T = T::default();
        while i != 0 {
            sum += self.table[i];
            i &= i - 1;
        }
        sum
    }

    fn add(&mut self, mut i: usize, x: T) {
        if i == 0 {
            panic!("0 is given to BIT");
        }
        while (i as usize) < self.table.len() {
            self.table[i as usize] += x;
            i += i - (i & (i - 1));
        }
    }
}
