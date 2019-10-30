use std::io; // brings in the standard input/output module

// this is the main function
fn main() {
    
    println!("Guess the number");

    println!("Please input your guess.");

    // create a new mutable variable of type string
    let mut guess = String::new();

    /* 
    use the standard library to call read line on the line inputted by user
    the & indicates that the argument is a reference to var in memory
    the .expect() lets us handle errors
     */
    io::stdin().read_line(&mut guess)
        .expect("Failed to Read line");

     println!("You guessed: {}", guess);
}
