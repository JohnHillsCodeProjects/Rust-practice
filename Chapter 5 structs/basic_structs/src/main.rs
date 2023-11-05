struct User { //Define structs like this
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
} //Structs should be defined in the global scope. Struct definitions will go out of scope and become invalid.

struct Copyable { f1: bool, f2: u16, f3: f32, f4: char, f5:[i32;3] }

struct PointIn3D(i32,i32,i32);

fn main() {
    println!("Rust supports structs. They are similar to tuples in that they can contain multiple values of different data types.");
    println!("However, they are different because you can assign labes to the values in a struct and can make methods for structs.");
    let mut test_user = User { //You can instance a struct and assign values to it like this
        active: false,
        username: String::from("muffmuncher_69"),
        email: String::from("cootchieslayer9000@emailaddress.com"),
        sign_in_count: 1
    };
    test_user.active = true; //You can edit single fields of a struct like this, assuming that struct is mutable.
    println!("\n~Displaying manually set user:"); display_user_details(&test_user);

    let test_username = String::from("this_is_a_string");
    let gened_user = create_user("testemail@string.literal", &test_username);
    println!("\n~Displaying user created with function:"); display_user_details(&gened_user);

    let inheriting_user = User {
        sign_in_count: 17,
        ..clone_user(&gened_user)
        //Going last and with the .. prefix indicates that any remaining field are to be the same as the inhereted user
        //Have to clone because, since the struct contains heap variables, it won't deep copy be default and will invalidate the old user
    };
    println!("\n~Displaying user that inherits from another user:"); display_user_details(&inheriting_user);

    println!("\n~Testing copying struct without method");
    let test_copy = Copyable { f1:true, f2: 86, f3: 7.29, f4: 'K', f5:[4,5,6] };
    print!("test_copy  = "); display_copyable(&test_copy);
    let actual_copy = Copyable { ..test_copy };
    print!("actual_copy = "); display_copyable(&actual_copy);
    println!("Can still access test_copy: {}", test_copy.f4);
    println!("This is because Copable only contains none-heap variables like integers and char");

    let test_point = PointIn3D(5, 4, -3);
    display_3d_point(&test_point);
    println!("I can still select from the struct because it wasn't moved: test_point.2 = {}", test_point.2);

    struct UnitStruct; //This is a unit struct. It is an empty struct that holds zero bytes of data. You can apply traits to them
    let empty_struct = UnitStruct;
    println!("You can define unit structs, which are empty structs that store 0 bytes, but can have traits applied to them.");
}

fn display_user_details(user: &User) {
    println!("Currently active: {}\nUsername: {}\nE-mail: {}\nSign-in count: {}", user.active, user.username, user.email, user.sign_in_count);
}

fn display_copyable(s: &Copyable) { println!("{}, {}, {}, {}, {:?}", s.f1, s.f2, s.f3, s.f4, s.f5); }

fn create_user(email_input:&str, username_input:&str) -> User {
    let email = email_input.to_string();       //Converts a string reference to string
    let username = username_input.to_string(); //Converts a string reference to string
    User {
        active: false,
        email,          //If you have variables named the same as the field, then you can use this shorthand instead of email:email
        username,       //Ditto
        sign_in_count: 1
    } //Here,the fields are out of order, but it doesn't matter since they are named and can be assigned in any order.
}

fn clone_user(taking_user: &User) -> User { 
    User {
        active: taking_user.active,
        username: taking_user.username.clone(),
        email: taking_user.email.clone(),
        sign_in_count: taking_user.sign_in_count
    }
} //This is a good way to clone a user, but you can attach it to the struct as a method (look in a different project in the same folder)

fn display_3d_point(p:&PointIn3D) { println!("(x,y,z) = ({},{},{})", p.0, p.1, p.2); }