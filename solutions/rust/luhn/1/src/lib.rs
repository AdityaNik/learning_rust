/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut v: Vec<u32> = Vec::new();

    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        } else if let Some(d) = c.to_digit(10) {
            v.push(d);
        } else {
            return false;
        }
    }

    if v.len() <= 1 {
        return false;
    }

    for i in (0..v.len()).rev().skip(1).step_by(2) {
        let temp = v[i] * 2;
        v[i] = if temp > 9 { temp - 9 } else { temp };
    }

    let sum: u32 = v.iter().sum();

    sum % 10 == 0
}
