#[derive(Debug)]

struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn display_sizes(&self) { print!("width: {}, height: {}", self.width, self.height); }
    fn area_method(&self) -> u32 { self.width * self.height }
    fn width(&self) -> bool { self.width > 0 } //The attribute can still be selected by just ommitting the ()
    fn square(s:u32) -> Self { Rectangle { width: s, height: s } }
}

fn main() {
    let x:u32 = 8; let y:u32 = 5;
    println!("\nUsing just 2 integer variables, the dimensions are ({}, {}) and the area is {}.", x, y, area_basic(x,y));
    let dimensions = (8,5);
    println!("Using a tuple, the dimensions are {:?} and the area is {}.", dimensions, area_tuple(dimensions));
    let rect = Rectangle { width: 8, height: 5 };
    println!("Using a struct, the dimensions are ({}, {}) and the area is {}.", rect.width, rect.height, area_struct(&rect));
    println!("Above, the values of the struct are printed individually, instead of trying to print the struct directly.");
    println!("This is because the \"Display\" trait is not implemented for the struct, but you can do it without the trait.");
    println!("You first have to use the string interpolation key {{:?}} and also derive debugging information with #[derive(Debug)]");
    println!("Printing struct:\nrect = {:?}", rect);
    println!("Use {{:#?}} to print each label on a new line:\nrect = {:#?}", rect);
    println!("You can also use the debug macro dbg!() to print the struct:");
    dbg!(&rect); //Must reference rect here instead of using the variable directly, for ownership reasons (this is unlike the print macro)

    println!("\nAn even better way of displaying a struct and of calculating the structs information is to create a struct method.");
    println!("These methods are defined using the \"impl\" key word for the struct, and then place the method declaration.");
    print!("Printing method: "); rect.display_sizes(); println!(" | Using a method, the area is {}", rect.area_method());
    println!("You can create methods that have the same names as the attributes, and to select the attribute you just parentheses.");
    println!("Is the width attribute, with a value of {}, more than 0: {}", rect.width, rect.width());

    println!("\nYou can create an \"associated function\" which is a function that does not take the struct as an argument.");
    println!("These functions are usually used for constructors.");
    let square_rect = Rectangle::square(25);
    print!("A square, defined with the square constructor in the rectangle struct: "); square_rect.display_sizes(); println!();
}

fn area_basic(x:u32, y:u32) -> u32 { x*y }

fn area_tuple(size: (u32, u32)) -> u32 { size.0 * size.1 }

fn area_struct(q: &Rectangle) -> u32 { q.width * q.height }