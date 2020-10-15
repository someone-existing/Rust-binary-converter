use std::io;
use text_io::read;

fn main() {
    println!("1. Binary to decimal");
    println!("2. Decimal to binary");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Couldn't read input");

    a = String::from(a.trim());

    if a == "1" {
        println!("Input binary number:");
        let n: i32 = read!();
        println!("In decimal form: {}", bin_to_dec(n));
    } else if a == "2" {
        println!("Input decimal number:");
        let n: i32 = read!();
        println!("In binary form: {}", dec_to_bin(n));
    }
}

fn bin_to_dec(mut n: i32) -> i32 {
    let mut dec = 0;
    let mut i = 0;
    let mut rem: i32;
    while n != 0 {
        rem = n % 10;
        n /= 10;
        dec += rem * 2i32.pow(i);
        i += 1;
    }
    return dec;
}

fn dec_to_bin(mut n: i32) -> i32 {
    let mut bin = 0;
    let mut rem: i32;
    let mut i = 1;
    while n != 0 {
        rem = n % 2;
        n /= 2;
        bin += rem * i;
        i *= 10;
    }
    return bin;
}
