pub fn is_armstrong_number(num: u32) -> bool {
    let mut number_of_digits = 0;

    let mut temp = num;

    while temp > 0 {
        number_of_digits += 1;
        temp /= 10;
    }

    let mut temp2 = num;
    let mut sum = 0;
    while temp2 > 0 {
        let number = temp2 % 10;
        sum += number.pow(number_of_digits);
        temp2 /= 10;
    }

    if sum != num {
        return false;
    }
    
    true
}
