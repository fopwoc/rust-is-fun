use num::bigint::Sign;
use num::BigInt;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::io;
use std::str::FromStr;

fn main() {
    println!("fibonacci sequence");

    loop {
        println!("Please fibonacci number:");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        // let n = Decimal::from_str(num.trim()).unwrap();
        //
        // let mut f = dec!(0);
        // let mut s = dec!(1);
        // let mut i = dec!(0);

        let n = BigInt::from_str(num.trim()).unwrap();

        let mut f = BigInt::new(Sign::Plus, vec![0]);
        let mut s = BigInt::new(Sign::Plus, vec![1]);
        let mut i = BigInt::new(Sign::Plus, vec![0]);

        while i < n {
            // i += dec!(1);
            // let result = f + s;
            // f = s;
            // s = result;
            //
            // println!("#{i}: {result}")

            i += BigInt::new(Sign::Plus, vec![1]);
            let result = f + &s;
            f = s;
            s = result;

            if i == n {
                println!("#{i}: {}", s)
            }
        }
    }
}
