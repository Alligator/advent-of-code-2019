use std::fs;

fn count_char(text: &str, ch: char) -> i32 {
    text.chars().filter(|x| *x == ch).count() as i32
}

fn parse_input(text: &str, width: i32, height: i32) -> Vec<&str> {
    let layer_size = (width * height) as usize;
    let mut output = Vec::new();

    for i in (0..text.len()).step_by(layer_size) {
        let substr = &text[i..i + layer_size];
        output.push(substr);
    }

    output
}

fn main() {
    let width = 25;
    let height = 6;

    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let image = parse_input(input.trim(), width, height);
    let mut fewest_zeroes: (i32, usize) = (99999, 0); // count, index

    for (idx, layer) in image.iter().enumerate() {
        let num_zeroes = count_char(&layer, '0');
        if num_zeroes < fewest_zeroes.0 {
            fewest_zeroes = (num_zeroes, idx);
        }
    }

    let layer = image[fewest_zeroes.1];
    println!("part 1: {}", count_char(&layer, '1') * count_char(&layer, '2'));

    // part 2
    // 0 - black
    // 1 - white
    // 2 - transparent

    let mut output = vec!['2'; (width * height) as usize];
    input
        .chars()
        .enumerate()
        .for_each(|(idx, c)| {
            let array_index = idx % (width * height) as usize;
            // still transparent
            if output[array_index] == '2' {
                output[array_index] = if c == '0' { ' ' } else { c }
            }
        });
    
    println!("part 2:");
    for line in output.chunks(width as usize) {
        let line_str: String = line.iter().collect();
        println!("{}", line_str);
    }
}
