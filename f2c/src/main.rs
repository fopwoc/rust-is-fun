use std::io;

fn main() {
    println!("f2c - fahrenheit to celsius");

    loop {
        println!("Please fahrenheit degree:");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: usize = fahrenheit
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let celsius:f32 = ((fahrenheit - 32) as f32 * 5.0) / 9.0;

        println!("It is {celsius} °C")
    }
}
