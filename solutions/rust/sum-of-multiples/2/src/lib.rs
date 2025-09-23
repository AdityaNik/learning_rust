pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut ans = 0;
    let mut factors_sum: Vec<u32> = Vec::new();
    if factors.is_empty() {
        return ans;
    }
    
    for i in 0..factors.len() {
        let mut temp = factors[i];
        let mut j = 2;
        while temp < limit {
            factors_sum.push(temp);
            temp = factors[i] * j;
            j += 1;
        }
    }
    factors_sum.sort();
    factors_sum.dedup();

    for i in 0..factors_sum.len() {
        ans += factors_sum[i];
    }
    ans
}
