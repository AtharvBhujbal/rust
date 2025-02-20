// Primitive data types

fn main(){
    // 1. Integers
    let x:i32 = -10;
    let y:u32 = 20;
    println!("x = {}", x);
    println!("y = {}", y);
    
    
    // 2. Floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 2.0;
    println!("f1 = {} {}", f1, f2);
    println!("f2 = {}", f2);

    // 3. Boolean
    let b1: bool = true;
    let b2: bool = false;
    println!("b1 = {}", b1);
    println!("b2 = {}", b2);

    // 4. Characters
    let c1: char = 'a';
    let c2: char = 'b';
    println!("c1 = {}", c1);
    println!("c2 = {}", c2);

    // Compound Data types
    // Array, tuples, slices, and strings(slice string)

    // 1. Arrays 
    let numbers:[i32; 5] = [1,2,3,4,5];
    println!("Numbers: {:?}",numbers);
    // ////////////////////////////////////////////////

    // 2. Tuples
    let tuple: (&str, i32, bool) = ("Alice", 20, true);
    println!("Tuple: {:?}", tuple);
    // ////////////////////////////////////////////////

    // 3. Slices
    let num_slice: &[i32] = &[1,2,3,4,5];
    println!("Slice: {:?}", num_slice);
    // ////////////////////////////////////////////////

    // 4. Strings
    let str1: &str = "Hello";
    let str2: String = String::from("World");
    println!("String1: {}", str1);
    println!("String2: {}", str2);
    // ////////////////////////////////////////////////


}