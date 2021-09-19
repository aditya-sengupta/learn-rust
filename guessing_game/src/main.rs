use std::io; // standard input/output library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut i = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // let - create variable
        // mut - mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        i = i + 1;

        // stdin - accept input
        // read_line - appends stdin to the string passed in as argument
        // & - pass by reference, access data without recopying it into memory
        // references are immutable which is why we put the & on mut
        // expect - error catching

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        // expect -> match, move from catching to handling errors
        // shadowing of the variable "guess"

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if i == 1 {
                    println!("Took 1 guess");
                } else {
                    println!("Took {} guesses", i);
                }

                break;
            },
        }
    }
}
