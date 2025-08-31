pub fn square_of_sum(n: u32) -> u32 {
    ((n * (n + 1)) / 2) * ((n * (n + 1)) / 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    let sq_of_sum = square_of_sum(n);
    let sum_of_sq = sum_of_squares(n);
    sq_of_sum - sum_of_sq
}
