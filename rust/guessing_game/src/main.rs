use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 * rand::Rng needed to be added to dependencies in Cargo.toml
 * For more information on the documentation of the packages you included
 * you can run:
 *
 *      $ cargo doc --open
 *
 */

fn main() {
    println!("Guess the number!");

    /*
     * thread_rng() gives us the particular rng that we're going to use
     * gen_range(star..end) gives us a range of the form:
     *
     *      start..end
     *
     * With start being inclusive and end being exclusive
     */

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        /*
         * Objects are mutable by default
         * Using mut makes them mutable
         */

        let mut guess = String::new();

        /*
         * This is how to get input from the user
         * Using &guess creates a reference
         * References are also immutable
         * Thus, we use &mut guess to make it mutable
         *
         * It is recommended to use a newline to split
         * up methods with the .method_name() syntax
         */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        /*
         * When we compare, we see that a String and an integer cannot be compared
         * We can fix this by doing the following below
         * We tell it it's an unsigned 32 bit integer
         * Notice that we created a variable with the same name as guess
         * Thankfully, Rust has something called shadowing
         *
         * trim() on a String instance will eliminate any whitespace at the
         * beginning and end
         *
         * parse() parses a string into a number, we specify what number when
         * we do u32
         *
         * We can then handle invalid input using match
         *
         * We return Ok if it's a number
         * And an Error if it's anything but a number, the _ is a wildcard
         * Just like Ocaml!
         */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        /*
         * This is how to compare the user's guess
         * against the random number
         *
         * We break away from the program if we win
         */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
