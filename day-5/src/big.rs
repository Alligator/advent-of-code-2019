use std::fs;
use std::io;
use std::io::Write;

/*
= opcodes =
add     1,X,Y,D     $D = $X + $Y
mul     2,X,Y,D     $D = $X * $Y
store   3,D         $D = user input
print   4,X         print $X
halt    99          halt

= parameter modes =
0 - position mode   - parameter is a memory address
1 - immediate mode  - parameter is a value

= opcode format =
example opcode:
    ABCDE X Y D
     1002,4,3,4

    DE = opcode    = 02
    C  = mode of X = 0 (position)
    B  = mode of Y = 1 (immediate)
    A  = mode of D = 0 (position)

    mem[4] = mem[4] * 3
*/

#[derive(Debug, PartialEq, Clone, Copy)]
enum Param {
    Position(i64),
    Immediate(i64),
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Store,
    Print,
    Halt,
    JmpIfTrue,
    JmpIfFalse,
    LessThan,
    Equal,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: Opcode,
    params: Vec<Param>,
}

fn parse_params(opcode: i64, params: &[i64]) -> Vec<Param> {
    let mut params_with_mode = Vec::<Param>::new();

    // two digit, all params are position
    if opcode < 100 {
        for param in params {
            params_with_mode.push(Param::Position(*param));
        }

        return params_with_mode;
    }

    // divide by 100 to chop off the opcode
    let mut remaining = opcode / 100;

    for param in params {
        if remaining == 0 {
            // we ran out of modes, always use position
            params_with_mode.push(Param::Position(*param));
        } else {
            let right_digit = remaining % 10;
            match right_digit {
                0 => params_with_mode.push(Param::Position(*param)),
                1 => params_with_mode.push(Param::Immediate(*param)),
                _ => (),
            }
            remaining /= 10;
        }
    }

    return params_with_mode;
}

fn parse_instruction(program: &[i64]) -> (Instruction, i64) {
    let mut prog_iter = program.iter().copied();

    let current = prog_iter.next();
    while current.is_some() {
        // opcode mod 100 to just get the right two digits;
        let opcode = current.and_then(|x| Some(x % 100));

        let mut new_opcode = Opcode::Halt;
        let mut param_count = 0;
        match opcode {
            Some(1) => {
                new_opcode = Opcode::Add;
                param_count = 3;
            },
            Some(2) => {
                new_opcode = Opcode::Mul;
                param_count = 3;
            },
            Some(3) => {
                new_opcode = Opcode::Store;
                param_count = 1;
            },
            Some(4) => {
                new_opcode = Opcode::Print;
                param_count = 1;
            },
            Some(5) => {
                new_opcode = Opcode::JmpIfTrue;
                param_count = 2;
            },
            Some(6) => {
                new_opcode = Opcode::JmpIfFalse;
                param_count = 2;
            },
            Some(7) => {
                new_opcode = Opcode::LessThan;
                param_count = 3;
            },
            Some(8) => {
                new_opcode = Opcode::Equal;
                param_count = 3;
            },
            Some(99) => {
                new_opcode = Opcode::Halt;
                param_count = 0;
            },
            _ => (),
        }

        let mut params = Vec::new();
        for _ in 0..param_count {
            params.push(prog_iter.next().unwrap());
        }

        let instr = Instruction {
            opcode: new_opcode,
            params: parse_params(current.unwrap(), &params),
        };
        println!("parsed: {:?} {:?} {:?}", current, instr, param_count);
        return (instr, param_count + 1);
    }

    let instr = Instruction {
        opcode: Opcode::Halt,
        params: Vec::new(),
    };
    return (instr, 1);
}

fn get_param_value(param: Param, program: &[i64]) -> i64 {
    match param {
        Param::Position(pos) => program[pos as usize],
        Param::Immediate(val) => val,
    }
}

// destinations are always immediate
fn get_param_dest(param: Param) -> i64 {
    match param {
        Param::Position(pos) => pos,
        Param::Immediate(val) => val,
    }
}

fn run_program(program: &[i64]) {
    let mut index = 0;
    let mut mem: Vec<i64> = Vec::from(program);
    // mem.resize(10000, 0);

    while index < mem.len() {
        let (instr, consumed) = parse_instruction(&mem[index..]);
        let mut jumped = false;

        println!("index: {}", index);
        match instr.opcode {
            Opcode::Add => {
                // add     1,X,Y,D     $D = $X + $Y
                let x = get_param_value(instr.params[0], &mem);
                let y = get_param_value(instr.params[1], &mem);
                let d = get_param_dest(instr.params[2]);
                mem[d as usize] = x + y;
                println!(
                    "add\t{:?}\t-> {}\n\t{:?}\t-> {}\n\t${}\t\t-> {}",
                    instr.params[0],
                    x,
                    instr.params[1],
                    y,
                    d,
                    mem[d as usize],
                );
            },
            Opcode::Mul => {
                // mul     2,X,Y,D     $D = $X * $Y
                let x = get_param_value(instr.params[0], &mem);
                let y = get_param_value(instr.params[1], &mem);
                let d = get_param_dest(instr.params[2]);
                mem[d as usize] = x * y;
                println!(
                    "mul\t{:?}\t-> {}\n\t{:?}\t-> {}\n\t${}\t\t-> {}",
                    instr.params[0],
                    x,
                    instr.params[1],
                    y,
                    d,
                    mem[d as usize],
                );
            },
            Opcode::Store => {
                // store   3,X,D       $D = X
                // let x = 1;
                print!("input> ");
                io::stdout().flush().expect("error flushing stdout!");

                let mut inp = String::new();
                io::stdin().read_line(&mut inp).expect("error reading stdin!");

                let x = inp
                    .trim()
                    .parse::<i64>()
                    .unwrap();

                let d = get_param_dest(instr.params[0]);
                mem[d as usize] = x;
                println!(
                    "store\t{}\n\t${}\t\t-> {}",
                    x,
                    d,
                    mem[d as usize],
                );
            },
            Opcode::Print => {
                // print   4,X         print $X
                let x = get_param_value(instr.params[0], &mem);
                println!("print\t{:?}\t-> {}", instr.params[0], x);
                println!("{}", x);
            },
            Opcode::JmpIfTrue => {
                let x = get_param_value(instr.params[0], &mem);
                let d = get_param_value(instr.params[1], &mem);
                if x != 0 {
                    index = d as usize;
                    jumped = true;
                }
                println!(
                    "jpt\t{:?}\t-> {}\n\t${}",
                    instr.params[0],
                    x,
                    d,
                );
            },
            Opcode::JmpIfFalse => {
                let x = get_param_value(instr.params[0], &mem);
                let d = get_param_value(instr.params[1], &mem);
                if x == 0 {
                    index = d as usize;
                    jumped = true;
                }
                println!(
                    "jpf\t{:?}\t-> {}\n\t${}",
                    instr.params[0],
                    x,
                    d,
                );
            },
            Opcode::LessThan => {
                let x = get_param_value(instr.params[0], &mem);
                let y = get_param_value(instr.params[1], &mem);
                let d = get_param_dest(instr.params[2]);
                mem[d as usize] = if x < y { 1 } else { 0 };
                println!(
                    " lt\t{:?}\t-> {}\n\t{:?}\t-> {}\n\t${}\t\t-> {}",
                    instr.params[0],
                    x,
                    instr.params[1],
                    y,
                    d,
                    mem[d as usize],
                );
            },
            Opcode::Equal => {
                let x = get_param_value(instr.params[0], &mem);
                let y = get_param_value(instr.params[1], &mem);
                let d = get_param_dest(instr.params[2]);
                mem[d as usize] = if x == y { 1 } else { 0 };
                println!(
                    " eq\t{:?}\t-> {}\n\t{:?}\t-> {}\n\t${}\t\t-> {}",
                    instr.params[0],
                    x,
                    instr.params[1],
                    y,
                    d,
                    mem[d as usize],
                );
            },
            Opcode::Halt => {
                println!("halted!");
                return;
            },
        }
        println!("");

        if !jumped {
            index += consumed as usize;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let program: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    
    println!("running program");
    run_program(&program);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Param::*;
    use Opcode::*;

    fn assert_parse_params(opcode: i64, params: &[i64], expected: &[Param]) {
        let result = parse_params(opcode, &params);
        assert_eq!(result, expected);
    }

    fn assert_parse_instruction(program: &[i64], expected: Instruction) {
        let (result, _consumed) = parse_instruction(&program);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_params_simple() {
        assert_parse_params(1, &vec![3, 4, 4], &vec![Position(3), Position(4), Position(4)]);
    }

    #[test]
    fn test_parse_params_modes() {
        assert_parse_params(102, &vec![3, 4, 4], &vec![Immediate(3), Position(4), Position(4)]);
        assert_parse_params(1102, &vec![3, 4, 4], &vec![Immediate(3), Immediate(4), Position(4)]);
        assert_parse_params(0102, &vec![3, 4, 4], &vec![Immediate(3), Position(4), Position(4)]);
    }

    #[test]
    fn test_parse_instruction() {
        // add
        assert_parse_instruction(
            &vec![1, 3, 4, 4],
            Instruction { opcode: Add, params: vec![Position(3), Position(4), Position(4)] },
        );

        // mul
        assert_parse_instruction(
            &vec![102, 1, 2, 3],
            Instruction { opcode: Mul, params: vec![Immediate(1), Position(2), Position(3)] },
        );

        // store
        assert_parse_instruction(
            &vec![3, 12],
            Instruction { opcode: Store, params: vec![Position(12)] },
        );
    }
}
