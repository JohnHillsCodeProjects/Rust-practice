//Tasks of chapter 3:
//Convert temperatures between celcius and fahrenheit 
//Find the nth number of the fibonacci sequence
//Print the 12 days of christmas

fn main() {
    test_temp_convert();
    test_fibonacci();
    test_twelve_days();
}

fn temp_convert(num:f32, symbol:char) {
    let mut cel:f32 = 0.0; 
    let mut fah:f32 = 0.0; 
    let mut kel:f32 = 0.0;
    let mut display:bool = true;
    if symbol.to_ascii_uppercase() == 'C' {
        cel = num;
        kel = cel + 273.15;
        fah = (1.8 * cel) + 32.0;
    } else if symbol.to_ascii_uppercase() == 'K' {
        kel = num;
        cel = kel - 273.15;
        fah = (1.8 * cel) + 32.0;
    } else if symbol.to_ascii_uppercase() == 'F' {
        fah = num; 
        cel = (fah - 32.0)/1.8;
        kel = cel + 273.15;
    } else { 
        println!("Invalid character input. Please use c, k or f to denote celcius, kelvin, and fahrenheit respectively.");
        display = false;
    } 
    if display { println!("{cel} celcius\n{fah} fahrenheit\n{kel} kelvin"); }
}

fn test_temp_convert() {
    let nums = [45.4586, 300.93, 28.0, 200.567];
    let chars = ['c','K','f','l'];
    println!(""); let mut n:f32; let mut c:char;
    for i in 0..4 {
        n = nums[i]; c = chars[i];
        println!("~temp_convert({n},'{c}')");
        temp_convert(n, c);
    }
}

fn fibonacci_for_loop(n:u32) -> u32 {
    let mut nums:[u32;2] = [0,1];
    if n == 0 || n == 1 {return nums[n as usize]} //Cannot index an array with anything other than usize or isize
    
    let mut temp_num: u32 = 0;
    for _i in 1..n { 
        temp_num = nums[0] + nums[1]; //Next number in the sequence
        nums[0] = nums[1]; //Move the last number back
        nums[1] = temp_num;  //Place the next number at the end.
    }
    return nums[1];
}

fn fibonacci_recurssive(n:u32) -> u32 {
    if n == 0 {0} else if n == 1 {1}
    else { fibonacci_recurssive(n-1) + fibonacci_recurssive(n-2) }
}

fn test_fibonacci() {
    let nums = [0, 1, 2, 3, 4, 5, 6, 17, 35]; 
    println!("\nTesting fibonacci:");
    let mut resultA:u32; let mut resultB:u32;
    for a in nums {
        resultA = fibonacci_for_loop(a);
        resultB = fibonacci_recurssive(a);
        println!("n = {a} | for loop = {resultA} | recurssive = {resultB}");
    }
}

fn twelve_days_of_christmas(mut limit:i32) {
    if (0 > limit) || (limit > 12) { limit = 12; } //Make the limit 12 if it's under zero
    let items = ["partridge in a pear tree","turtle doves","French hens","calling birds","gold rings","geese a-laying","swans a-swimming","maids a-milking","ladies dancing","lords a-leaping","pipers piping","drummers drumming"];
    let mut prefix:&str;
    let mut out:&str;
    for i in 1..=limit {
        print!("On the {i}"); 
        prefix = if i == 1 { "st" } else if i == 2 { "nd" } else if i == 3 { "rd" } else { "th" };
        println!("{prefix} day of Christmas my true love gave to me");
        for j in (1..=i).rev() { 
            if (j==1) && (i == 1) { print!("A "); } else if j == 1 { print!("And a "); } else { print!("{j} "); }
            out = items[(j-1) as usize]; println!("{out}");
        }
    } 
}

fn test_twelve_days() {
    let limits = [-3,0,1,2,5,14];
    println!("\nTesting the twelve days of christmas:");
    for l in limits {
        println!("\nLimit = {l}:");
        twelve_days_of_christmas(l);
    }
}