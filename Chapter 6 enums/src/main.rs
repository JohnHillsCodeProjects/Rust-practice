#[derive(Debug)]

enum JustEnumTypes { TypeFirst, TypeSecond, TypeLast }
//Here an enum is defined with no values attached to its variants

struct EmptyStruct;

enum ContainingEnums {
    OneInteger(i32),
    TwoFloats(f32,f32),
    CharUnitStructChar(char, EmptyStruct, char),
    Words(String),
    Fields{ c1:char, c2:char },
    EmptyActually
}

fn main() {
    println!("\n~Pattern matching empty enums:");
    let empty_enums = [
        JustEnumTypes::TypeFirst,
        JustEnumTypes::TypeSecond,
        JustEnumTypes::TypeLast
    ];
    for e in empty_enums {
        match e {
            JustEnumTypes::TypeFirst  => println!("Cycled to the first enum."),
            JustEnumTypes::TypeSecond => println!("Cycled to the second enum, which would be next."),
            JustEnumTypes::TypeLast   => println!("Cycled to the last enum, and ending the loop afterwards.")
        }
    } 

    println!("\n~Pattern matching non-empty enums:");
    let full_enums = [
        ContainingEnums::OneInteger(98),
        ContainingEnums::EmptyActually,
        ContainingEnums::TwoFloats(3.46, 223.68),
        ContainingEnums::Fields { c1: 'f', c2: 'F' },
        ContainingEnums::CharUnitStructChar('E', EmptyStruct, 'S'),
        ContainingEnums::Words(String::from("A string in an enum.")),
    ];
    for e in full_enums {
        match e {
            ContainingEnums::OneInteger(x) => println!("The single int is {}", x),
            ContainingEnums::TwoFloats(x,y) => println!("The 2 decimals are {} and {}", x, y),
            ContainingEnums::CharUnitStructChar(a,_,b) => //The underscore is used because the struct is not needed here
                println!("There are 2 characters surrounding an empty struct are {} and {}", a, b),
            //Lines of code can be placed on a new line as long as the => and , are included
            ContainingEnums::Words(st) => println!("The contained string is \"{}\"", st),
            ContainingEnums::Fields { c1, c2 } => println!("The 2 characters in the enum's fields are {} and {}", c1, c2),
            ContainingEnums::EmptyActually => println!("An empty enum")
        }
    }

    println!("\n~Pattern matching for variable assignment with else and code blocks.");
    let mut combo_string = String::new();
    let assignment_enums = [
        ContainingEnums::OneInteger(0),
        ContainingEnums::CharUnitStructChar('#', EmptyStruct, 'N'),
        ContainingEnums::Fields { c1: 'B', c2: '#' },
        ContainingEnums::EmptyActually
    ];
    for e in assignment_enums {
        let (char1,char2) = match e {
            ContainingEnums::CharUnitStructChar(a,_,b) => {
                println!("2 characters and a struct were found");
                (a,b)
            } ContainingEnums::Fields { c1, c2 } => {
                println!("2 characters and a struct were found");
                (c1,c2)
            } _ => ('_','_') //In the the other cases, return underscores 
        };
        combo_string.push(char1); 
        combo_string.push(char2);
    } println!("Combo_string = {combo_string}");

    println!("\n~If let statements.");
    let assignment_enums = [ //Have to redefine variable since Rust's compiler moves the value because of its use in the for loop
        ContainingEnums::OneInteger(0),
        ContainingEnums::CharUnitStructChar('#', EmptyStruct, 'N'),
        ContainingEnums::Fields { c1: 'B', c2: '#' },
        ContainingEnums::EmptyActually
    ];
    println!("Simple if let with one branch for assigning values");
    for e in assignment_enums {
        if let ContainingEnums::CharUnitStructChar(a,_,b) = e { println!("The 2 chars found are {} and {}", a, b); }
    }
    let assignment_enums = [ //Have to redefine variable since Rust's compiler moves the value because of its use in the for loop
        ContainingEnums::OneInteger(0),
        ContainingEnums::CharUnitStructChar('#', EmptyStruct, 'N'),
        ContainingEnums::Fields { c1: 'B', c2: '#' },
        ContainingEnums::EmptyActually
    ];
    println!("If let with more branches and an else case, used for variable assignment");
    combo_string = String::new(); //Restart combo string for this example
    for e in assignment_enums {
        let (one, two) = if let ContainingEnums::CharUnitStructChar(a,_,b) = e { (a,b) }
        else if let ContainingEnums::Fields { c1, c2 } = e { (c1,c2) }
        else { ('_','_') };
        combo_string.push(one); 
        combo_string.push(two);
    } println!("Combo_string = {combo_string}");
}