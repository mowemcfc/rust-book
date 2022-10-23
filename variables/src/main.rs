const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Seconds in five hours: {FIVE_HOURS_IN_SECONDS}");

    shadow(13);
    expression();
}

fn shadow(x: i32) {
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}


fn expression() {
    let y = {
        let x = 1;
        x + 1 // no semicolon implies this is an expression
        // expressions don't have semicolons
    };
    println!("The value of y is: {y}");
}
