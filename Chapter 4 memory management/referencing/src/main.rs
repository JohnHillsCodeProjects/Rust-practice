fn main() {
    println!("\n~Referencing:");
    println!("A reference is like a pointer in that it's an address we can follow to access owned data.");
    println!("The difference is that references will always point to a valid value during it's lifetime.");
    let mut s1 = String::from("this is a sentence+=+");
    let mut len = find_length(&s1);
    println!("The length of \"{}\" is {}", s1, len);
    println!("Here, Rust doesn't move ownership of s1, and will detect at compile-time when s1 isn't referenced.");
    println!("References are also immutable by default in order to stop unwnated modification. They can be made mutable.");
    add_to_string(&mut s1);
    len = find_length(&s1);
    println!("After adding, the length of \"{}\" is {}", s1, len);
    println!("There are restrictions on where references for data can exist in a scope. This is to prevent a data race.");
    println!("(more info can be found in the rust notes word doc)");
    let r1 = &len; let r2 = &len; //Multiple references that don't cause a compile error because none of them are mutable.
    println!("let r1 = &len creates {r1} | let r2 = &len creates {r2}");
    println!("You can print the memory address of a variable by using curly brackets and putting :p behind a reference.");
    println!("s1 address {:p}",&s1);
    println!("Referencing can also be used to allow arrays to be used in functions and also returned by those function.");
    let test_array: [i32; 4] = [5,10,-4,6]; //Array of 4 32-bit integers.
    println!("Declaring test_array = [5,-10,-4,6]");
    let new_array = display_and_clear(&test_array);
    println!("The new array has a size of {}", new_array.len());
}

fn find_length(s: &String) -> usize { s.len() }

//fn add_to_string(s: &String) { s.push_str("this was added later"); } //This function doesn't work because the reference is immutable

fn add_to_string(s: &mut String) { s.push_str("this is an addition"); } //This changes the string on the heap by using a mutable reference

//fn dangling_pointer() -> &String { let s = String::from("hello"); &s } 
//This function returns a reference to a string, but the string goes out of scope when the function ends.
//This would normally return a dnagling pointer, which is an invalid pointer, but Rust's compiler will throw an error instead.

fn display_and_clear(arr:&[i32]) -> &[i32] { //Takes an array reference as an input and returns array reference.
    //let mut elem:i32; //Current element of array
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    } 
    return &[];
} //Referencing allows you to take an array as an input and then return an array.