// I allowed unused to get rid of the error message while developing
// also implemented rust's standard library to be able to get users input
#![allow(unused)]
use std::io;

fn main() {
    let number = read_number();
    let word_size = read_word_size();

    if number >= 0 {
        // here I used "Some" so that no error is thrown for invalid inputs
        if let Some(bin) = unsigned_binary(number, word_size) {
        println!("Unsigned representation: {}", bin);
    } else {
        println!("Can't be represented as unsigned binary");
    }
    }
}

fn unsigned_binary(num: i32, word_size: u32) -> Option<String> {
    if num < 0 {
        return None;
    }

    // max is used to present the upper limit of the word size
    let max = 2_i32.pow(word_size) - 1;

    if num > max {
        return None;
    }

    //this is to format the output as binary
    Some(format!("{:0width$b}", num, width = word_size as usize))
}

fn read_number() -> i32 {
    println!("Enter a number: ");

    // assigning a new string for input
    let mut input = String::new();

    //then reading the input here
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().expect("Invalid number")
}

fn read_word_size() -> u32 {
    println!("Choose a word size (4 - 8 - 16): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<u32>().expect("Invalid word size");

    // using a match that return true in the allowed cases and panics for an invalid case
    match size {
        4 | 8 | 16 => size,
        _ => panic!("word size must be 4, 8, or 16")
    }

}