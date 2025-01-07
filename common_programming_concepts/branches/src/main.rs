fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true.");
    } else {
        println!("Condition is false.");
    }

    // if number { // expected `bool`, found integer
    //     println!("Number was three.");
    // }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let new_number = if number != 3 { 6 } else { 3 };
    // let new_number = if number != 3 { 6 } else { "three" }; // `if` and `else` have incompatible types
    println!("The value of new number is {new_number}");

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value of element is {element}")
    }

    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");

    println!("Exercises");
    println!("---");

    fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    println!(
        "32 degrees fahrenheit are {} degrees celsius.",
        fahrenheit_to_celsius(32.0)
    );

    fn return_nth_fibonacci(n: u32) -> u32 {
        if n <= 1 {
            return n;
        }

        return_nth_fibonacci(n - 1) + return_nth_fibonacci(n - 2)
    }

    println!("The sixth fibonacci number is {}.", return_nth_fibonacci(6));

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in days {
        println!("On the {day} day of Christmas...")
    }
}
