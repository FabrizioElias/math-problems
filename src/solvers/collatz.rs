pub fn collatz(limit: u64) {
    println!("Which starting number, under {}, produces the longest chain? Let's find out!", limit);
    let array_size: usize = limit as usize;

    let mut sequence_length: u64 = 0;
    let mut starting_number: u64 = 0;
    let mut sequence: u64;

    //we must use vec! for this array to be stored in the heap (it might be too big for the stack)
    let mut cache = vec![0; array_size];
    cache[1] = 1;

    for current in 2..limit {
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