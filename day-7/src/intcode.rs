use std::io;

pub struct VM {
  mem: Vec<i32>,
  instruction_ptr: usize,
  halted: bool,
}

impl VM {
  fn read(&mut self) -> i32 {
    let value = self.mem[self.instruction_ptr];
    self.instruction_ptr += 1;
    // println!("INT: read 1: {}", value);
    return value;
  }

  fn read2(&mut self) -> (i32, i32) {
    let values = (
      self.mem[self.instruction_ptr],
      self.mem[self.instruction_ptr + 1],
    );
    self.instruction_ptr += 2;
    // println!("INT: read 2: {}, {}", values.0, values.1);
    return values;
  }

  fn read3(&mut self) -> (i32, i32, i32) {
    let values = (
      self.mem[self.instruction_ptr],
      self.mem[self.instruction_ptr + 1],
      self.mem[self.instruction_ptr + 2],
    );
    self.instruction_ptr += 3;
    // println!("INT: read 3: {}, {}, {}", values.0, values.1, values.2);
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

  pub fn run(&mut self, input_buffer: &[i32]) -> Vec<i32> {
    let mut output_buffer = Vec::new();

    while !self.halted {
      let buf = self.run_until_input(input_buffer);
      output_buffer.extend(buf);
    }

    output_buffer
  }

  pub fn run_until_input(&mut self, input_buffer: &[i32]) -> Vec<i32> {
    let mut input_iter = input_buffer.iter();
    let mut output_buffer = Vec::new();

    loop {
      let instruction = self.read();
      let opcode = instruction % 100;
      let mode1 = instruction / 100 % 2;
      let mode2 = instruction / 1000 % 2;

      // println!("INT: {:?}", self.mem);
      // println!("INT: running {}", opcode);
      match opcode {
        1 => {
          let (x, y, d) = self.read3();
          let result = self.get_value(x, mode1).checked_add(self.get_value(y, mode2));
          if let Some(sum) = result {
            // println!("INT: sum, writing {}", sum);
            self.write(sum, d);
          } else {
            println!("overflow detected!");
            break;
          }
        },
        2 => {
          let (x, y, d) = self.read3();
          let result = self.get_value(x, mode1).checked_mul(self.get_value(y, mode2));
          if let Some(sum) = result {
            // println!("INT: mul, writing {}", sum);
            self.write(sum, d);
          } else {
            println!("overflow detected!");
            break;
          }
        },
        3 => {
          let d = self.read();

          if let Some(x) = input_iter.next() {
            self.write(*x, d);
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
          // println!("{}", self.get_value(s, mode1));
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
          // println!("halted!");
          self.halted = true;
          break;
        },
        _ => {
          println!("ERROR: unknown opcode {}", opcode);
          self.halted = true;
          break;
        },
      }

      // println!("");
    }

    output_buffer
  }

  pub fn from_program(program: &[i32]) -> VM {
    VM {
      mem: Vec::from(program),
      instruction_ptr: 0,
      halted: false,
    }
  }

  pub fn is_halted(&self) -> bool {
    self.halted
  }
}