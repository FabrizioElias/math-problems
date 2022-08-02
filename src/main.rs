extern crate stopwatch;
use stopwatch::Stopwatch;
use std::io::{Write, stdin, stdout};
mod prime_helpers;

fn main() {
    let mut choosen = String::new();
    let stdin = stdin();
    print_menu();
    loop {
        stdin.read_line(&mut choosen).expect("Failed to read line");
        println!();
        match choosen.to_lowercase().as_str().trim() {
            "1" => {
                println!("Collatz took {}ms", timed_execution(collatz).as_millis());
            }
            "2" => {
                println!("Prime summation (simple) took {}ms", timed_execution(prime_sum_simple).as_millis());
            }
            "3" => {
                println!("Prime summation (Sieve of Erastothenes) took {}ms", timed_execution(prime_sum_sieve_erastothenes).as_millis());
            }
            "x" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }
        print_menu();
        choosen.clear();
    }
}

fn print_menu() {
    println!();
    println!("Choose your problem and press enter (X to exit):");
    println!("1- Collatz conjecture (https://projecteuler.net/problem=14)");
    println!("2- Prime sum up to 2mil (simple) (https://projecteuler.net/problem=10)");
    println!("3- Prime sum up to 2mil (Sieve of Erastothenes) (https://projecteuler.net/problem=10)");
    print!("> ");
    stdout().flush().expect("Failed to flush STDOUT");
}

fn timed_execution(function_to_time: fn()) -> std::time::Duration { 
    let sw = Stopwatch::start_new();
    function_to_time();
    sw.elapsed()
}

fn collatz() {
    println!("Which starting number, under one million, produces the longest chain? Let's find out!");
    const NUMBER_LIMIT: i64 = 1000000;
    const ARRAY_SIZE: usize = 1000000;

    let mut sequence_length: i64 = 0;
    let mut starting_number: i64 = 0;
    let mut sequence: i64;

    //we must use vec! for this array to be stored in the heap (it's too big for the stack)
    let mut cache = vec![-1; ARRAY_SIZE];
    cache[1] = 1;

    for current in 2..NUMBER_LIMIT {
        sequence = current;
        let mut steps = 0;
        while sequence != 1 && sequence >= current {
            steps += 1;
            if (sequence % 2) == 0 {
                sequence /= 2;
            }
            else {
                sequence = sequence * 3 + 1;
            }
        }
        //Store result in cache
        cache[current as usize] = steps + cache[sequence as usize];

        //Check if sequence is the best solution
        if cache[current as usize] > sequence_length {
            sequence_length = cache[current as usize];
            starting_number = current;
        }
    }
    println!("Number {} has {} steps in it's chain.", starting_number, sequence_length);
}

fn prime_sum_simple() {
    const LIMIT: i64 = 2000000;
    println!("Whats is the sum of all primes from 0 to {} according to a trial division?", LIMIT);
    let mut sum: i64 = 0;
    for current in 2..LIMIT {
        if prime_helpers::is_prime(current as i64) {
            sum += current;
        }
    }
    println!("Sum of primes from 0 to {} = {}.", LIMIT, sum);
}

fn prime_sum_sieve_erastothenes() {
    const LIMIT: i64 = 2000000;
    println!("Whats is the sum of all primes from 0 to {} according to a sieve of Erastothenes?", LIMIT);
    let limit_sqrt: i64 = (LIMIT as f64).sqrt() as i64 + 1;
    let mut sum: i64 = 0;
    //we must use vec! for this array to be stored in the heap (it's too big for the stack)
    let mut primes = vec![true; (LIMIT + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    //0 and 1 are not primes, and 2 is prime, so we start reading the array from position 2 so we can mark its multiples as non-primes
    for index in 2..limit_sqrt {
        //if prime, change true to false at every position that is a multiple of index, excluding index
        if primes[index as usize] {
            let mut not_prime_index: usize = (index * index) as usize;
            while not_prime_index < LIMIT as usize {
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
    println!("Sum of primes from 0 to {} = {}.", LIMIT, sum);
}