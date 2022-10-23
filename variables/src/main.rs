const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Seconds in five hours: {FIVE_HOURS_IN_SECONDS}");

    shadow(13);
    expression();
    let x = plus_one(14);
    println!("The value of x is: {x}");

    let condition = false;
    let y = if condition { 4 } else { 99 };
    println!("The value of y is: {y}");

    // Fails: rust must know the type of this variable at compile time
    //  which is clearly impossible if it's assigned type is conditional
    //let z = if condition { "hello" } else { 13 };
    //println!("The value of z is: {z}");

    nested_loops_labeled();
}

fn nested_loops_labeled() {
    let mut innercounter = 0;
    let mut outercounter = 0;

    'counting_up: loop {
        outercounter += 1;
        println!("outer: {outercounter}");
        loop {
            innercounter += 1;
            println!("inner: {innercounter}");
            if innercounter == 20 {
                break 'counting_up;
            }
            if innercounter % 5 == 0 {
                break;
            }
        }
    }
}

fn loopy() {
    let mut counter = 0;    

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }; // no semicolon bc expression
    };
    println!("The result is {result}");
}

fn shadow(x: i32) {
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn expression() {
    let y = {
        let x = 1;
        x + 1 // no semicolon implies this is an expression
        // expressions don't have semicolons
    };
    println!("The value of y is: {y}");
}
