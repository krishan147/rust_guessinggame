use std::io; /*input output library*/

fn main() { /*function with no parameters*/
    println!("Guess the Number!");

    let mut guess = String::new(); /*mutable string so it can change the STring::new(); part returns a new instance of a string*/

    io::stdin().read_line(&mut guess) /*used to get input from the user and place it into a string*/
        .expect("Failed to read line"); /*what prints if no value is typed by user*/

    println!("You guessed {}",guess);

}