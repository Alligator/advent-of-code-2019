use std::str::FromStr;
use std::fs;

mod intcode;
use intcode::VM;

extern crate num;
use num::bigint::BigInt;
use num::{One, FromPrimitive};

fn main() {
    // let program = "104,1125899906842624,99";
    let program = fs::read_to_string("input.txt").unwrap();

    let program_bigints: Vec<BigInt> = program
        .trim()
        .split(",")
        .map(|x| BigInt::from_str(x).unwrap())
        .collect();

    // part 1
    let mut vm1 = VM::from_program(&program_bigints);
    let output1 = vm1.run(&[BigInt::one()]);

    print!("part 1: ");
    for num in output1 {
        print!("{} ", num);
    }
    println!("");

    // part 2
    let mut vm2 = VM::from_program(&program_bigints);
    let output2 = vm2.run(&[FromPrimitive::from_i32(2).unwrap()]);

    print!("part 2: ");
    for num in output2 {
        print!("{} ", num);
    }
    println!("");
}
