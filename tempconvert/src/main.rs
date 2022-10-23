use std::io;

fn main() {
    println!("Input your temperature in celcius");
    loop {
        let mut temp: String = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature!");
                continue;
            }
        };
        println!("Entered temp: {temp} celcius");
        let converted: f32 = convert(temp);
        println!("Converted temp: {converted} fahrenheit")
    }


}

fn convert(temp: f32) -> f32 {
    return temp * 1.8 + 32.0;
}
