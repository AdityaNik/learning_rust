pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 1;
    let mut num = 3;

    loop {
        if is_prime(num) {
            count += 1;
            if count == n + 1 {
                return num;
            }
        }
        num += 2;
    }
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }

    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
