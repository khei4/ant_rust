fn mod_pow(n: usize, mut e: usize, m: usize) -> usize {
    let mut base = n;
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res *= base;
            res %= m;
        }
        base *= base;
        base %= m;
        e >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn power_tests() {
        assert_eq!(mod_pow(2, 3, 5), 3);
        assert_eq!(mod_pow(2, 984939, 998244353), 655964744);
    }
}
