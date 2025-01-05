const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    println!("{THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is {x}");

    x = x + 1;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    // Shadowing vs reassigning -> useful for type change
    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len(); // gives error

    println!("spaces length {spaces}");
}
