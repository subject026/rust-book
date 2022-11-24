use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    println!("Please input your guess.");

    
    loop {        

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        // convert string input to int
        //      - trim() removes whitespace and beginning/end. Also removes newline character added
        //          when user hits enter
        //      - parse() does the conversion and requires a type annotation 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(value) => {
                println!("{value} is not a number!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");
        
        // cmp method returns a variant of the Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm has pattern to match and code to run if value fits
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
