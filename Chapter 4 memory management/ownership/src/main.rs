fn main() {

    println!("\n~How does scope work?\nThe variable s has not been defined yet");
    {
        let s = "S"; 
        println!("s was declared in a new scope (onto the stack) where s is the owner and the value is: {s}");
    } 
    println!("s has gone out of scope and the value has been dropped from the stack.");
    
    println!("\n~The string data type");
    let mut s = String::from("hello"); 
    println!("A string must be defined with the String namespace: {s}");
    let literal = ",world"; 
    println!("String literals can be defined from just quotation marks: {literal}");
    s.push_str(literal); //This adds a string literal to the end of a string
    println!("String literals can be added to strings: {s}");

    println!("\n~Dropping values");
    println!("String data types are stored on the heap, with the name of the variable being the intial pointer.");
    {
        let s = String::from("#[[]-#][]");
        println!("String declared inside scope: {s}");
    }
    println!("The variable goes out of scope and the value is dropped.\nIn other langauges this would cause a memory leak because of the heaped data,but Rust also drops the heaped data.");
    println!("This is done with the Rust function 'drop()' which is called automatically at the closing curly bracket.");

    println!("\n~Moving variables and data");
    let mut x = 5; let y = x;
    println!("Defining x and y with shallow copying : x = {x} | y = {y}");
    x += 2; 
    println!("Mutating x: x = {x} | y = {y}");
    println!("The value of y didn't change with x because it copied the value of x in that one instance.\nThis is because they scalar and stored on the stack.");
    println!("This also applies to arrays and tuples that contain types with defined sizes:\na = [0,23,45,7] and b = a");
    let mut a = [0,23,45,7]; let mut b = a; let mut a1 = a[1]; let mut b1 = b[1];
    println!("Initial values: a[1] = {a1} | b[1] = {b1}");
    a[1] = 35; b[1] = 67; a1 = a[1]; b1 = b[1];
    println!("Changed values: a[1] = {a1} | b[1] = {b1}");
    println!("Doing it with strings however won't work because this simple method will lead to 2 owners of the same value.\nThis is the same issue that you run into with lists in python. y = x means that any changes to one affects the other.");
    println!("So what Rust does is, if you attempt a shallow copy, it lets s2 own the list and then doesn't allow s1 to be used.");
    let mut s1 = String::from("valueEREST"); 
    println!("s1 declaration: {s1}");
    let mut s2 = s1;
    println!("s2 shallow copy: {s2}");
    //println!("{s1}"); //Line doesn't work because "borrow of moved value: s1"
    println!("Now s1 cannot be used because it's value was shallow copied to s2.");
    println!("This avoids the python list problem, as well as the 'double free' problem which can lead to memory corruption.");

    println!("\n~Copying variables and data");
    println!("It's extremely easy to copy data that is on the stack. Even a shallow copy will work");
    println!("However, if you copy the string back to s1 by using a deep copy method like .clone()");
    s1 = s2.clone();
    println!("You now get both variables: s1 = {s1} | s2 = {s2}");
    println!("This takes more time and memory than a shallow copy, but you get 2 independent copies.");
    println!("Meaning that any edits to s1 do not affect s2 and vice versa.");
    s1.push_str("_s_one"); s2.push_str("_s_two");
    println!("After editing both variables: s1 = {s1} | s2 = {s2}");

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    println!("\n~Functions and ownership");
    let s = String::from("hello"); 
    println!("Declare a string called s and a function that takes a string argument called some_string");
    println!("When the function is called, it calls some_string into scope, does stuff, and then drops it.");
    println!("But if you just use s as an argument in the function, you will pass owernship to some_string.\nSo when some_string is dropped, s no longer has a value and becomes invalid.");
    println!("Declared s = {s}");
    print!("takes_ownership(s) prints: ");
    takes_ownership(s);
    //println!("s after the function call: {s}"); //Invalid since s no longer has ownership
    println!("Now s no longer has ownership and is invalid. This is because strings are stored on the heap.");
    println!("This does NOT happen for values stored on the that are copied easily.");
    println!("Declared an integer called x and a function that takes an integer argument called some_integer.");
    let x = 5;
    print!("Value of x = {x}\nmakes_copy(x) prints: ");
    makes_copy(x);
    println!("After function call, x = {x}");
    println!("This is because integers don't require a special method to be copied since they are on the stack.");
    
    println!("\n~Returning and ownership");
    println!("When returning a value, the function gives ownership of that value to the variable being returned to.");
    println!("Defining a function that returns a string and declaring s1 with it.");
    let mut s1 = gives_ownership();
    println!("s1 = {s1}");
    println!("When a function takes a variable and then outputs a new variable, it essentially moves ownership.");
    println!("It takes ownership from the input, and gives ownership of the output to the output variable.");
    s1 = takes_and_gives_back(s1);
    println!("You can pass the value of s1 back to itself: s1 = {s1}");
    let s2 = takes_and_gives_back(s1);
    println!("Or you can pass the value of s1 to a new variable: s2 = {s2}");

} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens since the integer passed is copied.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string //some_string is returned and moves out to the calling function
} //some_string goes out of scope, but since it's value was moved, the string value itself isn't dropped

fn takes_and_gives_back(a_string: String) -> String { //a_string comes into scope as a parameter
    a_string  //a_string is returned and moves out to the calling function
} //a_string goes out of scope, but since it's value was moved, the string value itself isn't dropped