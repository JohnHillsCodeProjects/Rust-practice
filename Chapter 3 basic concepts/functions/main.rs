fn main() {
    println!("Hello, world!");
    another_function();
    parametered_function(50, 'G');
    let scoped_value = {
        let y = 16;
        y * 2
    }; //This scope returned y * 2 because there is no semi-colon there. The variable y goes out of scope and disappears.
    //The scope terminates in a semi-colon because the whole line acts as a statment, but the scope acts as an expression
    println!("Scoped value = {scoped_value}");
    let returned_value = returning_function(12);
    println!("Returned value = {returned_value}");
	let returned_value = returning_function_simpler(12);
	println!("Simpler returned value = {returned_value}");
	let (x,y,z) = tuple_return();
	println!("Tuple returned values: x = {x} | y = {y} | z = {z}");
}

fn another_function() { println!("Another function"); } //Even one line functions need the curly brackets

fn parametered_function(x:i32, y:char) { println!("Parameter values: x = {x}, y = {y}"); }

fn returning_function(x:i32) -> i32 { return x + 10 } //Use the return keyword to return a value

fn returning_function_simpler(x:i32) -> i32 { x + 10 } //If it's the last expression of the function, you can just remove the semi-colon

fn tuple_return() -> (i32, &'static str, bool) { //You can use a tuple to return multiple values of different types
	let ret0 = 86;
	let ret1 = "4_STR1NG_L1T3R4L";
	let ret2 = true;
	return (ret0,ret1,ret2);
}