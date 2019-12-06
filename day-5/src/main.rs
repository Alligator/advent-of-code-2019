use std::fs;
use std::io;

struct VM {
  mem: Vec<i32>,
  instruction_ptr: usize,
}

impl VM {
  fn read(&mut self) -> i32 {
    let value = self.mem[self.instruction_ptr];
    self.instruction_ptr += 1;
    return value;
  }

  fn read2(&mut self) -> (i32, i32) {
    let values = (
      self.mem[self.instruction_ptr],
      self.mem[self.instruction_ptr + 1],
    );
    self.instruction_ptr += 2;
    return values;
  }

  fn read3(&mut self) -> (i32, i32, i32) {
    let values = (
      self.mem[self.instruction_ptr],
      self.mem[self.instruction_ptr + 1],
      self.mem[self.instruction_ptr + 2],
    );
    self.instruction_ptr += 3;
    return values;
  }

  fn write(&mut self, value: i32, dest: i32) {
    self.mem[dest as usize] = value;
  }

  fn get_value(&self, value: i32, mode: i32) -> i32 {
    match mode {
      0 => self.mem[value as usize],
      _ => value,
    }
  }

  pub fn run(&mut self) {
    loop {
      let instruction = self.read();
      let opcode = instruction % 100;
      let mode1 = instruction / 100 % 2;
      let mode2 = instruction / 1000 % 2;

      match opcode {
        1 => {
          let (x, y, d) = self.read3();
          self.write(self.get_value(x, mode1) + self.get_value(y, mode2), d);
        },
        2 => {
          let (x, y, d) = self.read3();
          self.write(self.get_value(x, mode1) * self.get_value(y, mode2), d);
        },
        3 => {
          let d = self.read();

          let mut inp = String::new();
          io::stdin().read_line(&mut inp).expect("error reading stdin!");
          let x = inp.trim().parse::<i32>().unwrap();

          self.write(x, d);
        },
        4 => {
          let s = self.read();
          println!("{}", self.get_value(s, mode1));
        },
        5 => {
          let (x, d) = self.read2();
          if self.get_value(x, mode1) != 0 {
            self.instruction_ptr = self.get_value(d, mode2) as usize;
          }
        },
        6 => {
          let (x, d) = self.read2();
          if self.get_value(x, mode1) == 0 {
            self.instruction_ptr = self.get_value(d, mode2) as usize;
          }
        },
        7 => {
          let (x, y, d) = self.read3();
          self.write(
            if self.get_value(x, mode1) < self.get_value(y, mode2) { 1 } else { 0 },
            d,
          );
        },
        8 => {
          let (x, y, d) = self.read3();
          self.write(
            if self.get_value(x, mode1) == self.get_value(y, mode2) { 1 } else { 0 },
            d,
          );
        },
        99 => {
          println!("halted!");
          break;
        },
        _ => {
          println!("ERROR: unknown opcode {}", opcode);
          break;
        },
      }
    }
  }

  pub fn from_program(program: &[i32]) -> VM {
    VM {
      mem: Vec::from(program),
      instruction_ptr: 0,
    }
  }
}

fn main() {
  // let program = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
  let input = fs::read_to_string("input.txt").unwrap();

  let program: Vec<i32> = input
      .trim()
      .split(",")
      .map(|x| x.parse::<i32>().unwrap())
      .collect();

  let mut vm = VM::from_program(&program);
  vm.run();
}