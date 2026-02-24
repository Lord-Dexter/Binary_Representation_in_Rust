// I allowed unused to get rid of the error message while developing
// also implemented rust's standard library to be able to get users input
#![allow(unused)]
use std::io;

// had to use an enum to allow both float and integer because rust requires a specified function output type
enum Number {
        Integer(i32),
        Float(f64),
    }
fn main() {

    let num = read_number();
    let word_size = read_word_size();

    match num {
        Number::Integer(num) => {

        if num >= 0 {
            // here I used "Some" so that no error is thrown for invalid inputs
        if let Some(bin) = unsigned_binary(num, word_size) {
            println!("Unsigned representation: {}", bin);
        } else {
            println!("Can't be represented as unsigned binary");
        }
        }

        if let Some(bin) = signed_binary(num, word_size) {
            println!("Signed representation: {}", bin);
        }
        else {
            println!("Can't be represented as signed binary");
        }

        if let Some(bin) = ones_compliment(num, word_size) {
            println!("One's Complement representation: {}", bin)
        }
        else {
            println!("Can't be represented as one's complement");
        }

        if let Some(bin) = twos_compliment(num, word_size) {
            println!("two's Complement representation: {}", bin)
        }
        else {
            println!("Can't be represented as two's complement");
        }
    }
        Number::Float(num) => {

        let fixed = fixed_point(num, word_size);
        println!("{}", fixed);
    }
}

fn fixed_point(num: f64, word_size: u32) -> String {
    let int_bits = word_size / 2;
    let frac_bits = word_size - int_bits;

    // used trunc to get the integer part from the float
    let integer_part = num.trunc() as i32;

    // fract only gets the fraction part
    let fractional_part = num.fract().abs();

    let int_binary = format!("{:0width$b}", integer_part.abs(), width = int_bits as usize);
    let frac_binary = fractional_to_binary(fractional_part, frac_bits);

    format!("{}.{}", int_binary, frac_binary)
}

fn fractional_to_binary(mut fraction: f64, bits: u32) -> String {
    // initializing an empty string
    let mut result = String::new();


    // process here loops through the bits and checks the leading digit if 1.some_fraction it appends
    // one and appends 0 otherwise
    for _ in 0..bits{
        fraction *= 2.0;
        if fraction >= 1.0 {
            result.push('1');
            fraction -= 1.0;
        } else {
            result.push('0');
        }
    }
    result
}

fn twos_compliment(num: i32, word_size: u32) -> Option<String> {
    let max = 2_i32.pow(word_size - 1) -1;
    let min = -2_i32.pow(word_size - 1);

    if num > max || num < min {
        return None;
    }

    if num >= 0 {
        return Some(format!("{:0width$b}", num, width = word_size as usize));
    }

    // here I used bit shifting to represent two's compliment
    let twos = (1_i32 << word_size) + num;
    Some(format!("{:0width$b}", twos, width = word_size as usize))
}

fn ones_compliment(num: i32, word_size: u32) -> Option<String> {
    //limits of one's compliment
    let max = 2_i32.pow(word_size - 1) - 1;
    let min = -max;

    if num > max || num < min {
        return None;
    }

    let positive_binary = format!(
        "{:0width$b}",
        num.abs(),
        width = word_size as usize
    );

    if num >= 0 {
        Some(positive_binary)
    } else {
        let inverted: String = positive_binary
            .chars()
            // mapping every 0 character to a 1 and vice-versa
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect();
        Some(inverted)
    }
}

fn signed_binary(num: i32, word_size: u32) -> Option<String> {
    // here sign checks if the number is positive or negative before assigning
    let sign = if num < 0 { '1' } else {'0'};

    // abs funtion is used here to get the magnitude
    let count = num.abs();

    // here word size is subtracted to signify the lost bit
    let max = 2_i32.pow(word_size - 1) - 1;

    if count > max {
        return None;
    }

    let count_in_binary = format!("{:0width$b}", count, width = (word_size-1) as usize);

    Some(format!("{}{}", sign, count_in_binary))
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

fn read_number() -> Number {
    println!("Enter a number: ");

    // assigning a new string for input
    let mut input = String::new();

    //then reading the input here
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    // applied this to differentiate between floats and integers
    if trimmed.contains('.') {
        let value = trimmed.parse::<f64>().expect("Invalid float");
        Number::Float(value)
    } else {
        let value = trimmed.parse::<i32>().expect("Invalid integer");
        Number::Integer(value)
    }
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
}