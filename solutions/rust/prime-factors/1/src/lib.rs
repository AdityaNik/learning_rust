pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    while n % 2 == 0 {
        ans.push(2);
        n /= 2;
    }
    let mut divisor = 3;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            ans.push(divisor);
            n /= divisor;
        }
        divisor += 2;
    }
    if n > 1 {
        ans.push(n);
    }

    ans
}
