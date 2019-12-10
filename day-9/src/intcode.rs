extern crate num;

use num::bigint::BigInt;
use num::{FromPrimitive, ToPrimitive, Integer, Zero, One};

pub struct VM {
  mem: Vec<BigInt>,
  instruction_ptr: usize,
  relative_base: usize,
  halted: bool,
}

impl VM {
  fn read(&mut self) -> BigInt {
    let value = self.mem[self.instruction_ptr].clone();
    self.instruction_ptr += 1;
    return value;
  }
  fn read_value(&mut self, mode: i32) -> BigInt {
    let val = self.read();
    self.get_value(val, mode)
  }

  fn read2(&mut self) -> (BigInt, BigInt) {
    let values = (
      self.mem[self.instruction_ptr].clone(),
      self.mem[self.instruction_ptr + 1].clone(),
    );
    self.instruction_ptr += 2;
    return values;
  }
  fn read2_values(&mut self, mode1: i32, mode2: i32) -> (BigInt, BigInt) {
    let values = self.read2();
    (self.get_value(values.0, mode1), self.get_value(values.1, mode2))
  }

  fn read3(&mut self) -> (BigInt, BigInt, BigInt) {
    let values = (
      self.mem[self.instruction_ptr].clone(),
      self.mem[self.instruction_ptr + 1].clone(),
      self.mem[self.instruction_ptr + 2].clone(),
    );
    self.instruction_ptr += 3;
    return values;
  }
  fn read3_values(&mut self, mode1: i32, mode2: i32) -> (BigInt, BigInt, BigInt) {
    let values = self.read3();
    (
      self.get_value(values.0, mode1),
      self.get_value(values.1, mode2),
      values.2, // last param is always positional
    )
  }

  fn write(&mut self, value: BigInt, dest: usize) {
    if dest >= self.mem.len() {
      self.mem.resize(dest * 2, BigInt::zero());
    }

    self.mem[dest] = value;
  }

  fn get(&mut self, src: usize) -> BigInt {
    if src >= self.mem.len() {
      self.mem.resize(src * 2, BigInt::zero());
    }

    self.mem[src].clone()
  }

  fn get_value(&mut self, value: BigInt, mode: i32) -> BigInt {
    match mode {
      // positin
      0 => self.get(value.to_usize().unwrap()),

      // immediate
      1 => value,

      // relative
      2 => self.get(((self.relative_base as i32) + ToPrimitive::to_i32(&value).unwrap()) as usize),

      // unknown
      _ => value,
    }
  }

  fn get_addr(&self, value: BigInt, mode: i32) -> usize {
    let v = ToPrimitive::to_i32(&value).unwrap();
    match mode {
      // position, just return the address
      0 => v as usize,

      // relative, add the relative base
      2 => ((self.relative_base as i32) + v) as usize,

      // unknown
      _ => v as usize,
    }
  }

  pub fn run(&mut self, input_buffer: &[BigInt]) -> Vec<BigInt> {
    let mut output_buffer = Vec::new();

    while !self.halted {
      let buf = self.run_until_input(input_buffer);
      output_buffer.extend(buf);
    }

    output_buffer
  }

  pub fn run_until_input(&mut self, input_buffer: &[BigInt]) -> Vec<BigInt> {
    let mut input_iter = input_buffer.iter();
    let mut output_buffer = Vec::new();

    loop {
      let instruction = self.read();
      let instruction32 = ToPrimitive::to_i32(&instruction).unwrap();
      let opcode = instruction32 % 100;
      let mode1 = ((instruction32 / 100) % 10) % 3;
      let mode2 = ((instruction32 / 1000) % 10) % 3;
      let mode3 = ((instruction32 / 10000) % 10) % 3;

      // println!("INT: running {} {} {} {}", opcode, mode1, mode2, mode3);
      match opcode {
        1 => {
          let (x, y, d) = self.read3_values(mode1, mode2);

          let addr = self.get_addr(d, mode3);
          let result = x.checked_add(&y);

          if let Some(sum) = result {
            self.write(sum, addr);
          } else {
            println!("overflow detected!");
            break;
          }
        },
        2 => {
          let (x, y, d) = self.read3_values(mode1, mode2);

          let addr = self.get_addr(d, mode3);
          let result = x.checked_mul(&y);

          if let Some(sum) = result {
            self.write(sum, addr);
          } else {
            println!("overflow detected!");
            break;
          }
        },
        3 => {
          let d = self.read();
          let addr = self.get_addr(d, mode1);

          if let Some(x) = input_iter.next() {
            self.write(x.clone(), addr);
          } else {
            // woah buddy we ran outta input
            // walk the instruction ptr back to the 3 opcode
            self.instruction_ptr -= 2;
            return output_buffer;
          }
        },
        4 => {
          let s = self.read();
          output_buffer.push(self.get_value(s, mode1));
        },
        5 => {
          let (x, d) = self.read2();
          if !self.get_value(x, mode1).is_zero() {
            self.instruction_ptr = ToPrimitive::to_usize(&self.get_value(d, mode2)).unwrap();
          }
        },
        6 => {
          let (x, d) = self.read2();
          if self.get_value(x, mode1).is_zero() {
            self.instruction_ptr = ToPrimitive::to_usize(&self.get_value(d, mode2)).unwrap();
          }
        },
        7 => {
          let (x, y, d) = self.read3();

          let x_val = self.get_value(x, mode1);
          let y_val = self.get_value(y, mode2);
          let addr = self.get_addr(d, mode3);

          self.write(
            if x_val < y_val { BigInt::one() } else { BigInt::zero() },
            addr,
          );
        },
        8 => {
          let (x, y, d) = self.read3();

          let x_val = self.get_value(x, mode1);
          let y_val = self.get_value(y, mode2);
          let addr = self.get_addr(d, mode3);

          self.write(
            if x_val == y_val { BigInt::one() } else { BigInt::zero() },
            addr,
          );
        },
        9 => {
          let offset = self.read_value(mode1);

          // all of this type bullshit is to make sure everything is an i32 when
          // i do the add since once since could be negative, but a usize when i
          // store it
          let offset_val = ToPrimitive::to_i32(&offset).unwrap();
          let base = self.relative_base as i32;

          self.relative_base = (base + offset_val) as usize;
        },
        99 => {
          self.halted = true;
          break;
        },
        _ => {
          println!("ERROR: unknown opcode {}", opcode);
          self.halted = true;
          break;
        },
      }
    }

    output_buffer
  }

  pub fn from_program(program: &[BigInt]) -> VM {
    VM {
      mem: Vec::from(program),
      instruction_ptr: 0,
      relative_base: 0,
      halted: false,
    }
  }

  pub fn is_halted(&self) -> bool {
    self.halted
  }
}