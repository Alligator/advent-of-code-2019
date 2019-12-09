use std::fs;
use itertools::Itertools;

mod intcode;

fn part_one(program: &[i32]) -> i32 {
  let combinations = (0..=4).permutations(5);

  let mut best: (i32, Vec<i32>) = (0, Vec::new());

  for phase_settings in combinations {
    // println!("trying {:?}", phase_settings);

    let mut input = 0;
    for setting in &phase_settings {
      let mut vm = intcode::VM::from_program(&program);
      let output = vm.run(&[*setting, input]);
      input = *output.first()
        .expect("no output from a vm!");
    }

    if input > best.0 {
      best = (input, phase_settings);
    }
  }

  best.0
}

fn part_two(program: &[i32]) -> (i32, Vec<i32>) {
  let combinations = (5..=9).permutations(5);
  let mut best: (i32, Vec<i32>) = (0, Vec::new());

  for phase_settings in combinations {
    let mut first_run = true;
    let mut done = false;
    let mut input = 0;

    let mut vms = [
      intcode::VM::from_program(&program),
      intcode::VM::from_program(&program),
      intcode::VM::from_program(&program),
      intcode::VM::from_program(&program),
      intcode::VM::from_program(&program),
    ];

    while !done {
      for (vm, setting) in vms.iter_mut().zip(&phase_settings) {
        if first_run {
          // pass the phase
          let output = vm.run_until_input(&[*setting, input]);
          input = *output.first().unwrap();
        } else {
          let output = vm.run_until_input(&[input]);
          input = *output.first().unwrap();
        }
      }

      done = vms.iter().all(|x| x.is_halted());
      first_run = false;
    }

    if input > best.0 {
      best = (input, phase_settings);
    }
  }

  best
}

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();

  let program: Vec<i32> = input
      .trim()
      .split(",")
      .map(|x| x.parse::<i32>().unwrap())
      .collect();

  println!("running");
  println!("part 1: {}", part_one(&program));
  println!("part 2: {:?}", part_two(&program));
}
