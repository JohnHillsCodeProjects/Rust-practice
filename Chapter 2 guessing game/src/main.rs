use std::io; //Uses the input/output library
use rand::Rng; //This trait defines methods that random number generators implement 
use std::cmp::Ordering; //Use the ording function from the standard library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //Generates random integer from 1 upto and including 100

    println!("The secret number is: {secret_number}\n"); //String interpolation

    loop { //Starts an infinite loop that can be broken with break
        println!("Please input your guess.");

        let mut guess = String::new(); //Creates a mutable, empty string variable

        io::stdin() //Uses the standard input function from the io library 
            //(the new lines are not necessary but can help with readability)
            .read_line(&mut guess) //Reads from line that is typed by the user (similar to cin>> in c++)
            //This line also returns an enum called "Result". It has 2 variants which are Err and Ok. Err invokes .expect() and Ok doesn't
            .expect("Failed to read line"); //If the line cannot be read, then this line outputs the string as an error
            //If this line is not included, then the compiler throws an error because the Result output is not caught.

        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); 
        //This is to make guess an integer for pattern matching
        //trim() removes any whitespace at the start and end (guess ends in \n since you had to press enter to input)
        //parse() will attempt to change the variables type to whatever was specified in the declaration (which is u32 here)
        //expect() is for error handling if parse() cannot parse the value correctly 

        //The better version of that line of code looks like:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Please input a number"); continue; }
        };
        //Where parse() emits a Result value that has the 2 enum values. Here, they are handled through pattern matching
        //"continue" will not close the code on an error, and will instead, go to the next iteration of the loop. 

        println!("You guessed: {guess}"); //String interpolation

        //Pattern matching is explained in chapter 6 of the rust docs. 
        //You may be able to add more than 1 line to each arm of match.
        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; } //"break" will end the infinite loop. 
            //This is how to add 2 or more lines to pattern matching
        }
    }
}