fn main() {
    println!("Hello, world!");
    let vec_u8 = vec![4,6,10];
    let vec_char = vec!['v','5','!','o','P'];
    let vec_f32 = vec![0.98,7.8462,3.221];
    let u = largest(&vec_u8);
    let c = largest(&vec_char);
    let f = largest(&vec_f32);
    println!("Largest u8 in {:?} is {}",vec_u8, u);
    println!("Largest char in {:?} is {}", vec_char, c);
    println!("Largest float in {:?} is {}", vec_f32, f);
    let s = GenericStruct{ f1:9, f2:'R', f3:5.666 };
    //let e1 = GenericEnum::V1::<u8,_,_>(16);
    //let e2 = GenericEnum::V2::<i32,char,bool>('k');
    //let e3 = GenericEnum::V3::<bool,u16,f32>(7.29);
    let e1: GenericEnum<u8, _, _> = GenericEnum::V1::<u8,_,_>(16);
}

//fn largest<T>(list: &[T]) -> &T { //This definition has no ordering behvaiour defined, and then returns a compilation error
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { //The long-form of defing behaviour for the generic type
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct GenericStruct<A,B,C> { f1:A, f2:B, f3:C }
enum GenericEnum<A,B,C> { V1(A), V2(B), V3(C) }