use std::io; /*input output library*/
use std::cmp::Ordering;
use rand::Rng;

fn main() { /*function with no parameters*/
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); /* Generates a random no. between two numbers. */ 

    /* println!("The secret number is: {}", secret_number); */

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); /*mutable string so it can change the String::new(); part returns a new instance of a string*/

        io::stdin().read_line(&mut guess) /*used to get input from the user and place it into a string*/
            .expect("Failed to read line"); /*what prints if no value is typed by user*/

        let guess: u32 = match guess.trim().parse() { /*handles when text is input*/
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guessed {}",guess); /*prints what the user types in*/

        match guess.cmp(&secret_number) { /*match between guess and secret number*/
            Ordering::Less  => println!("Too small"),
            Ordering::Greater  => println!("Too big"),
            Ordering::Equal  => { println!("You win");
            break;
            }
            
        }
    }
}