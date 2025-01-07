fn main() {
    println!("Hello, world!");

    time_remaining(5, 'h');

    // Statements vs expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.
    // -> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

    let y = {
        let x = 4;
        x + 1
    };

    println!("The value of y is {y}.");

    let x = plus_one(five());

    println!("The value of x is {x}.");
}

fn time_remaining(t: i32, unit_label: char) {
    println!("{t}{unit_label} remaining.");
}

fn five() -> i32 {
    // return 5; // same
    5
}

fn plus_one(n: i32) -> i32 {
    n + 1
}
