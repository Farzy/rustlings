// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)


use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();
    let sumsums = Arc::new(Mutex::new(0)); // Does not need "mut"!

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        let child_sumsums = Arc::clone(&sumsums);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
            let mut ss = child_sumsums.lock().unwrap();
            *ss += sum;
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
    println!("The sum of sums is: {}", *sumsums.lock().unwrap());
}
