fn main() {
	let x:i32 = 47;
    println!("First declaration of x = {x}");
	let mut x:f32 = 7.84;
	println!("Second declaration of x = {x}");
	x = 3.24; 
	println!("Mutating x = {x}");
	let x:String = String::from("3rd");
	println!("Third declaration of x = {x}");
	let x:char = 'G';
	println!("Fourth declaration of x = {x}");
	println!("\nWhat happens when you redeclare a variable is that the new variable is added to memory, but the old one still exists.");
	println!("You just cannot access the old one, even if you drop the current one. However, the compiler still knows it's there.");
	println!("This means that when the current scope ends, the compiler drops the value in the proper way, and doesn't cause a memory leak.");
}
