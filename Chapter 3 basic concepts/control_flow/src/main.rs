#[allow (non_snake_case)]

fn main() {
    let Joe_is_valid = false;
    if Joe_is_valid { println!("Well that's a suprise."); } //If statements don't need parentheses around the boolean expression
    else { println!("Get in the garbage disposal, heathen!"); }

    let number = if Joe_is_valid {5} else {7}; //If statement value assignment
    println!("Number assigned from if statement = {number}");
    
    //Match statements are like switch statements in C++, but with a few key differences.
    //They can use ranges, or cases look a lot simpler, and you can match a string with some &str data by using the method .as_str()
    //And the performance differnces between match and if are basically non-existant because of the rust compiler optimising both to hell and back.
    println!("Tell me about {}", number);
    match number {  
        // Match a single value
        1 => println!("One!"),  
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
    //The valid data types 

    //Loops

    let mut i = 0;
    //Standard loops are infinite unless broken with breka.You can skip an iteration with continue
    loop { 
        if i == 6 { i += 1; continue; }
        else if i == 9 { break; }
        else { print!("{i}"); }
        i+=1;
    }

    i=0; let looped_number = loop {
        i += 1;
        if i == 10 { break i * 2; }
    };
    println!("\nNumber assigned from loop = {looped_number}");

    let mut in_loop = 0;
    let mut multiply = 0;
    'first_loop: loop {
        multiply += 1;
        loop {
            in_loop += 1;
            if (multiply == 3) && (in_loop % 5 == 3) { break 'first_loop; } //Breaks both loops
            else if in_loop % 5 == 0 { break; } //Breaks the second loop instead
        }
    }   
    let loop_in_loop_number = in_loop * multiply;
    println!("Double loop number output = {loop_in_loop_number}");

    print!("Regular while looping: ");
    i = 0; while i < 5 { i+=1; print!("{i}"); }

    print!("\nRegular for looping: ");
    for i in 0..5 { print!("{i}"); } //For loops can be done on already defined variables

    print!("\nIncremental for looping: ");
    for i in (0..7).step_by(2) { print!("{i}"); }

    print!("\nReversed for looping: ");
    for i in (0..5).rev() { print!("{i}"); }

    print!("\nReversed, incremental for looping: ");
    for i in (0..7).step_by(2).rev() { print!("{i}"); }

    print!("\nFor looping through array (with space): ");
    let numbers = [45, 56, 3, 70, 12, 89, 89, 100];
    for n in numbers { print!("{n} "); }
}

