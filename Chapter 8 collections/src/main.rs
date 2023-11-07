//You do not have to spcifically include vector or string because they are already preloaded in a new project.
use std::collections::HashMap; //Unlike vector and string, hashmaps have to be included with a use statement

fn main() {
    testing_vectors();
    testing_strings();
    testing_hashmaps();
}

fn testing_vectors() {
    let _v: Vec<i32> = Vec::new(); //Can create an empty vector with the new() function.
    let mut v: Vec<i8> = vec![4,35,8,9]; //Can create a vector with preset elements with the vec! macro. Can be an empty array.
    let mut append2v: Vec<i8> = vec![12,3,4,5]; //Array for appending. Will be emptied after appending.
    v.append(&mut append2v);
    v.push(15);
    v.insert(3,17);
    v[7] = 10; //Use this to change whatever is in index 7 to 
    print!("\nVector v = ");
    for i in &v { print!("{} ", i); } //Iterating over every element
    println!();
    for i in &v { println!("{:p} ", i); } //Iterating over every element and printing the address

    let item1 = &v[3];
    let item2 = v.get(4);
    //let item3 = &v[100]; //This line will panic because the index is too large
    let item4 = v.get(100); //This line won't panic, and instead returns a None.
    println!("\nItem at index 3: {}",item1);
    match item2 { 
        Some(t) => println!("Item at index 4: {}", t), 
        None => println!("No item is at index 4"), 
    }
    match item4 { 
        Some(t) => println!("Item at index 100: {}", t), 
        None => println!("No item is at index 100"), 
    }

    let first = &v[0];
    println!("The first element is {}",first);
    v.push(6);
    //println!("First = {}", first); //This causes a compiler error because v is a mutable reference and invalidates the immutable reference
    println!("Cannot access the first element refernce now since v has changed");
    let first = &v[0];
    println!("Must create a new reference. The first element is now {}",first);
}

fn testing_strings() {
    let data = "initial contents";
    let s = data.to_string(); //Can also use new() and then adding strings, or from()
    println!("\nA string created from a string literal: {}", s);
    println!("Both string literals and strings use UTF-8 encoding, so there are 1,112,064 valid character you can use.");
    let hello = String::from("السلام عليكم"); println!("Valid string data: {}", hello);
    let hello = String::from("Dobrý den");    println!("Valid string data: {}", hello);
    let hello = String::from("Hello");        println!("Valid string data: {}", hello);
    let hello = String::from("שָׁלוֹם");         println!("Valid string data: {}", hello);
    let hello = String::from("नमस्ते");         println!("Valid string data: {}", hello);
    let hello = String::from("こんにちは");    println!("Valid string data: {}", hello);
    let hello = String::from("안녕하세요");    println!("Valid string data: {}", hello);
    let hello = String::from("你好");         println!("Valid string data: {}", hello);
    let hello = String::from("Olá");          println!("Valid string data: {}", hello);
    let hello = String::from("Здравствуйте"); println!("Valid string data: {}", hello); 
    let hello = String::from("Hola");         println!("Valid string data: {}", hello);
    println!("Some of these may not print to the command line, but they can still be stored as strings perfectly fine.");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s2 = {} | s3 = {}",s2,s3);
    let s4 = " Extra!";
    let s1 = format!("{} {} |{}",s3,s2,s4);
    println!("Created s1 with format macro. s1 = {}\nFormated with the strings '{}', '{}', and '{}'",s1,s3,s2,s4);

    let st = "नमस्ते";
    println!("\nTesting the string \"{}\"", st);
    print!("Iterating through characters: ");
    for i in st.chars() { print!("{} ", i); }
    print!("\nIterating through bytes: ");
    for i in st.bytes() { print!("{} ", i); }

    let st = "Здравствуйте";
    println!("\nTesting the string \"{}\"", st);
    print!("Iterating through characters: ");
    for i in st.chars() { print!("{} ", i); }
    print!("\nIterating through bytes: ");
    for i in st.bytes() { print!("{} ", i); }
}

fn testing_hashmaps() {
    let mut scores:HashMap<&str,&str> = HashMap::new();
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    scores.insert(&blue, "10");
    scores.insert(&yellow, "50");
    //You can index a HashMap like you can with a vector. Invalid keys will cause a code panic.
    let indexed = scores["Blue"];
    println!("\nCreating a HashMap with string literal keys and items.\nValue indexed at key \"Blue\": {indexed}");
    let indexed = scores["Blue"]; //Repeating to see if the item was moved to this varaiable
    println!("Value indexed at key \"Blue\": {indexed}");
    //You can also find values with the .get() method. This returns an Option enum for safe error handling.
    //The .copied() method copies the contents of the Option if it has one.
    let got = scores.get("Yellow").cloned().unwrap_or("0");
    println!("Value found with get() with key \"Yellow\": {got}");
    let got = scores.get("Yellow").cloned().unwrap_or("0"); //Repeating to see if the item was moved to this varaible
    println!("Value found with get() with key \"Yellow\": {got}");
    //If the key is invalid, like here, then it'll return whatever value is inside the unwrap_or() method.
    let invalid_get = scores.get("Green").cloned().unwrap_or("0");
    println!("Value found with get() with key \"Green\": {invalid_get}");

    let mut scores:HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //You can index a HashMap like you can with a vector. Invalid keys will cause a code panic.
    let indexed = scores[&String::from("Blue")];
    println!("\nCreating a HashMap with String keys and integer values.\nValue indexed at key \"Blue\": {indexed}");
    //You can also find values with the .get() method. This returns an Option enum for safe error handling.
    //The .copied() method copies the contents of the Option if it has one.
    let got = scores.get(&String::from("Yellow")).copied().unwrap_or(0);
    println!("Value found with get() with key \"Yellow\": {got}");
    //If the key is invalid, like here, then it'll return whatever value is inside the unwrap_or() method.
    let invalid_get = scores.get(&String::from("Green")).copied().unwrap_or(0);
    println!("Value found with get() with key \"Green\": {invalid_get}");
    println!("Printing the HashMap: {:?}", scores);

    println!("\nAdding values to the HashMap\nYou can add values with insert() but this will overwrite any values with existing keys.");
    scores.insert(String::from("Red"), 35); //Inserts Red:35
    scores.insert(String::from("Blue"), 15); //Overwrites Blue to make Blue:15
    println!("You can also use entry(), which will add the value only if the key doesn't exist");
    scores.entry(String::from("Green")).or_insert(70); //Inserts Green:70
    scores.entry(String::from("Yellow")).or_insert(100); //Does nothing

    println!("\nSince entry() returns a referance to the value at that key, you can create a variable to contain that reference.");
    let obj = scores.entry(String::from("Green")).or_insert(0); //Returns a reference to Green's value
    *obj += 100; //Modifies the data at that referance.
    println!("This then allows you to edit the value in memory by dereferancing that referance.");

    println!("Iterating through the HashMap.");
    for (key, value) in &scores { println!("{key}: {value}"); }
}