use crate::prime_helpers; //this is possible because "mod prime_helpers.rs" is present in main.rs, so here we only need to use the crate

pub fn prime_sum_simple(limit: u64) {
    println!("Whats is the sum of all primes from 0 to {} according to a trial division?", limit);
    let mut sum: u64 = 0;
    for current in 2..limit {
        if prime_helpers::is_prime(current as i64) {
            sum += current;
        }
    }
    println!("Sum of primes from 0 to {} = {}.", limit, sum);
}

pub fn prime_sum_sieve_erastothenes(limit: u64) {
    println!("Whats is the sum of all primes from 0 to {} according to a sieve of Erastothenes?", limit);
    let limit_sqrt: i64 = (limit as f64).sqrt() as i64 + 1;
    let mut sum: i64 = 0;
    //we must use vec! for this array to be stored in the heap (it's too big for the stack)
    let mut primes = vec![true; (limit + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    //0 and 1 are not primes, and 2 is prime, so we start reading the array from position 2 so we can mark its multiples as non-primes
    for index in 2..limit_sqrt {
        //if prime, change true to false at every position that is a multiple of index, excluding index
        if primes[index as usize] {
            let mut not_prime_index: usize = (index * index) as usize;
            while not_prime_index < limit as usize {
                primes[not_prime_index] = false;
                not_prime_index += index as usize;
            }
        }
    }
    /*
    //This is one way to enumerate all items, by using a for
    for (index, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            sum += index as i64
        }
    }
    */

    //This is one way to enumerate all items, by using a for_each function with one argument (a tuple -> |x, y|)
    primes.iter().enumerate().for_each(|(index, is_prime)| {
        if *is_prime {
            sum += index as i64
        }
    });
    println!("Sum of primes from 0 to {} = {}.", limit, sum);
}