pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut ans = 0;
    let mut factors_sum: Vec<u32> = Vec::new();
    if factors.is_empty() {
        return ans;
    }
    
    for &factor in factors.iter() {
        let mut temp = factor;
        let mut j = 2;
        while temp < limit {
            factors_sum.push(temp);
            temp = factor * j;
            j += 1;
        }
    }
    factors_sum.sort();
    factors_sum.dedup();

    for &factor in factors_sum.iter() {
        ans += factor;
    }
    ans
}
