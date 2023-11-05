fn main() {
    let mut s1 = String::from("hello world");
    println!("The string variable s1 = {s1}");
    let index_s1 = first_word_index(&s1);
    println!("The index of the first space in s1 = {index_s1}");
    println!("While indexing a string in Rust is complicated, it's much eaiser to take a slice of the string.");
    let test_slice = &s1[4..8]; //Have to borrow a string to do a slice 
    println!("A random slice of s1 from 4 to 8: \"{test_slice}\"");
    let slice_s1 = first_word_slice(&s1);
    println!("Using slices to grab the first word of s1: \"{slice_s1}\"");
    s1.push_str(" there"); //Editing s1 returns a mutable reference to s1, and invalidates slice_s1
    //println!("Cannot print the s1 slice after s1 is cleared: slice_s1 = {slice_s1}"); //Because s1 is a reference, it cannot exist alongside the mutable reference
    let improved_slice_string = first_word_slice_improved(&s1);
    let improved_slice_str = first_word_slice_improved("hello world there");
    println!("Using an improved function that can take either a string literal or a reference to a string.");
    println!("With string input: \"{}\" | With string literal input \"{}\"", improved_slice_string, improved_slice_str);
    println!("You can also slice an array. Declaring arr = [9,4,2,-3,8,-14,9,0]"); 
    let arr = [9,4,2,-3,8,-14,9,0]; let arr_slice = slice_array(&arr);
    print!("Printing arr: "); print_array(&arr);
    print!("\nPrinting arr_slice: "); print_array(&arr_slice); print!("\n");
}

fn first_word_index(s: &String) -> usize {                    //This returns the index if the end of the first word of a string 
    let bytes = s.as_bytes();                          //Returns an array of bytes of the input string
    for (i, &item) in bytes.iter().enumerate() {   //This line is complicated, and more info is found in chapters 6 and 13
        if item == b' ' { return i; }                         //If a space is found, return the position 
    } s.len()                                                 //If no spaces are found, return the length of the string
} 
//This function is good, but if the original string goes out of scope or ets editied, then the number means nothing.
//And this method is even worse for trying to find the second word, because you need a start and end index that both have the same problem.

fn first_word_slice(s: &String) -> &str {                     //This returns a slice of the string that contains the first word.
    let bytes = s.as_bytes();                          //Returns an array of bytes of the input string
    for (i, &item) in bytes.iter().enumerate() {   //Same complicated line as before
        if item == b' ' { return &s[..i]; }                   //If a space is found, return a slice from the start to the index of the space
    } &s[..]                                                  //If no spaces can be found, return the whole word 
                                                              //Useful top: You don't have to include the 0 index start and .len() index end
}
//This function is much better, because it returns the actual first word, instead of an index. 
//Slices also make coding a function to find the second word much easier, since it outputs a slice and not 2 integers.

fn first_word_slice_improved(s: &str) -> &str { 
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    } &s[..]
} //Now the function definition has a string literal as the input, and can take both strings or string literals as an input,

fn slice_array(arr:&[i32]) -> &[i32] { &arr[2..5] } //You can also slice arrays.

fn print_array(arr:&[i32]) { for i in arr { print!("{},", i); } }