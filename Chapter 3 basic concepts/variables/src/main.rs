#[allow(non_snake_case)] //This is included so that rust-analyser doesn't complain when you use camel-case

fn main() {
    let x = 5; //This variable is immutable, meaning its value cannot be changed
    println!("x = {x}");
    //x = 6; //This line will return an error at compile time because it s trying to "mutate an immutable"
    let x = "hello"; //This line works because x is being redelcared with the let keyword
    println!("x = {x}");
    let mut y = 5; //This variable is mutable, menaing its value can change
    println!("y = {y}");
    y = 6; //Here you can change the value of y without an entire reassignment
    println!("y = {y}");
    //y = "world"; //This line will return an error at compile time because you cannot change a variable's type without redeclaration

    //Constants
    const CONST1:char = 'f'; //Constants can be assigned simple values
    println!("CONST1 = {CONST1}");
    const CONST2:i32 = 30 * 45 + 86; //Constants can be assigned sums like this since the value is found at compile time.
    println!("CONST2 = {CONST2}");
    //const CONST3:i32 = x * 5 + 4; //This won't work because the value can only be found at runtime

    //Shadowing and scope
    let scope1 = 5; //To be shadowed and not changed
    let mut scope2 = 7; //To be changed 
    println!("scope1 = {scope1} | scope2 = {scope2}");
    {
        let scope1 = scope1 * 15;
        scope2 += 5;
        let scope3 = 6;
        println!("Shadowing scope1, changing scope2, creating scope3 in scope");
        println!("scope1 = {scope1} | scope2 = {scope2} | scope3 = {scope3}");
    }
    println!("After scope\nscope1 = {scope1} | scope2 = {scope2}");
    //println!("After scope scope3 {scope3}"); //Does not return scope3 as it is now out of scope

    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------
    //Variable types

    //Scalar types (single values, the most primitive types)
    let Integer:u64 = 64323;  //An unsigned 64bit integer
    let Boolean:bool = false; //A boolean
    let Float:f64 = 38.62347; //A 64bit floating point number
    let Char:char = 'P';      //A character 
    println!("\nInteger = {Integer} | Boolean = {Boolean} | Float = {Float} | Char = {Char}");

    //Compound types (multiple values in one type)
    let Tuple:(char, i8, bool, f32) = ('E', -127, true, 3.45); //A tuple containing a character, 8bit signed integer, boolean, and 32bit floating point number
    //println!("Tuple = {Tuple}"); //Tuples cannot be displayed because they do not "implement std::fmt::display"
    let (item0, item1, item2, item3) = Tuple; //Select and assign all items of a tuple
    println!("Tuple items = {item0},{item1},{item2},{item3}");
    let FirstTupleItem = Tuple.0;  //Manual selection of tuple items
    println!("FirstTupleItem = {FirstTupleItem}");

    let Array = [34,52,486,756,908]; //An array containing integers (arrays are mutable so you can assign new values to indexes)
    //println!("Array = {Array}"); //Arrays cannot be displayed because they do not "implement std::fmt::display"
    let ArrayItem0 = Array[0];
    println!("ArrayItem0 = {ArrayItem0}");
    let mut arr:[i32; 4] = [0;4]; //Creates a 4 element array of just zeros.
    arr[0] = 3; //Assign elements individually
    arr[1] = 4;
    arr[2] = 6;
    arr[3] = 7;
    println!("Array prinitng {:?}",arr); //THis is how you print arrays
}
