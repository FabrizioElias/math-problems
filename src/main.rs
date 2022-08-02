extern crate stopwatch;
use stopwatch::Stopwatch;
use std::io::{Write, stdin, stdout};
mod prime_helpers;
mod solvers;

fn main() {
    let mut choosen = String::new();
    let stdin = stdin();
    print_menu();
    loop {
        stdin.read_line(&mut choosen).expect("Failed to read line");
        println!();
        match choosen.to_lowercase().as_str().trim() {
            "1" => {
                println!("Collatz took {}ms", timed_execution_with_argument(solvers::collatz::collatz, choose_limit()).as_millis());
            }
            "2" => {
                println!("Prime summation (simple) took {}ms", timed_execution_with_argument(solvers::prime_summation::prime_sum_simple, choose_limit()).as_millis());
            }
            "3" => {
                println!("Prime summation (Sieve of Erastothenes) took {}ms", timed_execution_with_argument(solvers::prime_summation::prime_sum_sieve_erastothenes, choose_limit()).as_millis());
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
    println!("Choose your problem and press enter          (X to exit):");
    println!("1- Collatz conjecture                        (https://projecteuler.net/problem=14)");
    println!("2- Prime summation (simple)                  (https://projecteuler.net/problem=10)");
    println!("3- Prime summation (Sieve of Erastothenes)   (https://projecteuler.net/problem=10)");
    print!("> ");
    stdout().flush().expect("Failed to flush STDOUT");
}

fn timed_execution_with_argument(function_to_time: fn(u64), argument: u64) -> std::time::Duration { 
    let sw = Stopwatch::start_new();
    function_to_time(argument);
    sw.elapsed()
}

fn choose_limit() -> u64 {
    let mut choosen_limit = String::new();
    print!("Choose a limit (number, greater than zero): ");
    stdout().flush().expect("Failed to flush STDOUT");
    stdin().read_line(&mut choosen_limit).expect("Failed to read line");
    choosen_limit.trim().parse::<u64>().expect("Cannot parse limit")
}