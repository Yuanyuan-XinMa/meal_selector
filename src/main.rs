use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Please enter an integer to guess");
    let random_number = rand::thread_rng().gen_range(1..=10);
    let mut correct_guess = false;
    let mut guessed_number: i32;
    while !correct_guess {
        let mut guessed_number_str = String::new();
        std::io::stdin()
            .read_line(&mut guessed_number_str)
            .expect("Cannot read input line from stdin");
        guessed_number = guessed_number_str
            .trim()
            .parse()
            .expect("Input is not an integer");
        match guessed_number.cmp(&random_number) {
            Ordering::Less => println!(
                "You guessed {}, which is smaller than the correct number",
                guessed_number
            ),
            Ordering::Greater => println!(
                "You guessed {}, which is larger than the correct number",
                guessed_number
            ),
            Ordering::Equal => correct_guess = true,
        }
    }
    println!("You guessed {}, CORRECT! CONGRATS!!", random_number);
}
