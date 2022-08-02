pub fn is_prime(number: i64) -> bool {
    if number == 2 {
        return true;
    }
    if number <= 1 || number % 2 == 0 {
        return false;
    }
    let limit: i64 = (number as f64).sqrt() as i64 + 1;
    for current in 3..limit {
        if number % current == 0 {
            return false;
        }
    }
    true
}