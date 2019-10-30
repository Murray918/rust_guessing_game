use rand::Rng;
use std::cmp::Ordering;
use std::io; // brings in the standard input/output module //

// this is the main function
fn main() {
    // generate a secret random number from the rand crate
    let secret_number = rand::thread_rng().gen_range(1, 101);

    /*
    the loop statement creates an infinite loop
    the loop stops with the break statement
    */
    loop {
        println!("Guess the number");

        println!("Please input your guess.");

        // create a new mutable variable of type string
        let mut guess = String::new();

        /*
        use the standard library to call read line on the line inputted by user
        the & indicates that the argument is a reference to var in memory
        the .expect() lets us handle errors
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read line");

        /*
        here we create a shadow variable named guess
        we use this shadow variable to change that type of the variable
        we cannot compare variables that do not have the same type together
        we declare that the shadow is a unsigned 32bit integer
        @.trim => removes the whitespace in an string
        @.parse => turns a string into some kind of number
        */
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // print the number the user has guessed
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // this breaks our infinite loop
            }
        }
    }
}
