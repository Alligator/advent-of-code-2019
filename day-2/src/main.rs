use std::fs;

fn run_program(program: &Vec<i64>) -> Vec<i64> {
    let mut mem = program.to_vec();
    let mut idx = 0;

    while idx < mem.len() {
        let opcode = mem[idx];
        // println!("op {} at {}", opcode, idx);
        match opcode {
            1 => {
                // add
                let x = mem[idx + 1] as usize;
                let y = mem[idx + 2] as usize;
                let dest = mem[idx + 3] as usize;
                // println!("  add. x: {} y: {} dest: {}", x, y, dest);
                mem[dest] = mem[x] + mem[y];
            },
            2 => {
                // mul
                let x = mem[idx + 1] as usize;
                let y = mem[idx + 2] as usize;
                let dest = mem[idx + 3] as usize;
                // println!("  mul. x: {} y: {} dest: {}", x, y, dest);
                mem[dest] = mem[x] * mem[y];
            },
            99 => {
                // halt
                break;
            }
            _ => panic!("unknown opcode {}", opcode),
        }
        idx += 4;
    }

    return mem;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| {
            x.parse::<i64>().unwrap()
        })
        .collect();
    
    // 1202 program alarm
    let mut copy1 = program.to_vec();
    copy1[1] = 12;
    copy1[2] = 2;
    let output = run_program(&copy1);

    println!("part 1: {}", &output[0]);

    let expected = 19690720;
    let mut copy = program.to_vec();

    for a in 0..99 {
        for b in 0..99 {
            copy[1] = a;
            copy[2] = b;
            let output = run_program(&copy);
            if output[0] == expected {
                println!("part 2: noun = {}, verb = {}, answer = {}", a, b, 100 * a + b);
                break;
            }
        }
    }
}
