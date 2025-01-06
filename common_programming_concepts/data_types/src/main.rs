use std::any::type_name_of_val;
use std::io;

fn main() {
    // Primitive data types
    // Scalar types
    // integers
    let n = 255; // i32 is the default -> 32bits
                 // 8-bit	i8	u8
                 // 16-bit	i16	u16
                 // 32-bit	i32	u32
                 // 64-bit	i64	u64
                 // 128-bit	i128	u128
                 // arch	isize	usize

    println!("{}", type_name_of_val(&n));

    // floating-point numbers
    let x: f32 = 2.0;
    println!("{x} {}", type_name_of_val(&x));
    let y = 3.0 / 0.43; // default f64 - higher precision
    println!("{y} {}", type_name_of_val(&y));

    // Numeric operations
    let addition = 5 + 10;
    let subtraction = 95.5 - 4.3;
    let multiplication = 4 * 30;
    let division_quotient = 56.7 / 32.2;
    let division_truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    println!(
        "addition: {addition}; 
subtraction: {subtraction}; 
multiplication: {multiplication}; 
division_quotient: {division_quotient}; 
division_truncated: {division_truncated}; 
remainder: {remainder}; 
        "
    );

    // Booleans
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
    // Characters - char type is four bytes
    let c = 'c';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // Compound types
    // Tuple
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("{a} {b} {c}; index '1' access: {}", tup.1);

    // let mut xyz: (i32, i32) = (1, 2);
    let mut xyz = (1, 2);
    xyz.0 = 3;
    xyz.1 += 1;

    println!("{}", xyz.0 == xyz.1);

    // Array
    // Fixed length and every element should have the same type.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.

    // Array shorthand
    let _a = [3; 5]; // [3, 3, 3, 3, 3]

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let second = months[1];

    println!("{first} {second}");

    println!("Please enter month index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let month = months[index];

    println!("The value of month at index {index} is {month}.");
}
