pub fn square(s: u32) -> u64 {
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    let mut ans: u64 = 0;
    let base: u64 = 2;
    for i in 0..64 {
        ans += base.pow(i as u32);
    }
    ans
}
