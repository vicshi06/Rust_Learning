use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // This creates a secret number from range 1 to 100 inclusive 
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // This reads the input from terminal 
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // 1. This parse the input into u32 if possible 
           // 2. If it is not able to parse the input into u32, it will continue 
           // 3. Trim is used to remove whitespace 
           // 4. Match is used to match if it is able to parse into u32 
    
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        } // The cmp method compares two values and can be alled on anything that can be compared 
          // Watch out for syntax and pretty cool! 
}
}
