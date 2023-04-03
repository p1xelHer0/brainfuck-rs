use std::io;
use std::io::Read;
use std::usize;

fn brainfuck(data: &str) -> Vec<u8> {
    let commands: Vec<u8> = data.as_bytes().to_vec();
    let mut instruction_pointer = 0;

    const CELL_SIZE: usize = 30000;
    let mut cells = [0; CELL_SIZE];
    let mut data_pointer = 0;
    let mut stack: Vec<usize> = Vec::new(); // vec![]? Does it matter? Opinions?

    let mut input = [0; 1];
    let mut output: Vec<u8> = Vec::new();

    fn bracket_match(instruction_pointer: &usize, commands: &Vec<u8>) -> usize {
        let mut depth = 1;
        let mut instruction_pointer = *instruction_pointer;
        while depth > 0 {
            instruction_pointer += 1;
            match commands.get(instruction_pointer) {
                Some(instruction) => match instruction {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    _ => (),
                },
                None => panic!(
                    "instruction pointer [{}] outside of commands",
                    instruction_pointer
                ),
            }
        }
        instruction_pointer
    }

    while instruction_pointer < commands.len() {
        match commands.get(instruction_pointer) {
            Some(instruction) => match instruction {
                b'>' => data_pointer += 1,
                b'<' => data_pointer -= 1,
                b'+' => cells[data_pointer] += 1,
                b'-' => cells[data_pointer] -= 1,
                b'[' => match cells.get(data_pointer) {
                    Some(0) => instruction_pointer = bracket_match(&instruction_pointer, &commands),
                    None => panic!("Data pointer [{}] outside of cells", data_pointer),
                    _ => stack.push(instruction_pointer),
                },
                b']' => match cells.get(data_pointer) {
                    Some(0) => _ = stack.pop(),
                    None => panic!("Data pointer [{}] outside of cells", data_pointer),
                    _ => match stack.last() {
                        Some(next_pointer) => instruction_pointer = *next_pointer,
                        None => panic!("No matching `[` found to jump to"),
                    },
                },
                // user input doesn't work right now?
                b',' => match io::stdin().read_exact(&mut input) {
                    Ok(_) => {
                        cells[data_pointer] = input[0];
                    }
                    _ => panic!("Failed to read user input"),
                },
                b'.' => match cells.get(data_pointer) {
                    Some(cell_content) => output.push(*cell_content),
                    None => (),
                },
                _ => (),
            },
            None => panic!(
                "Instruction pointer [{}] outside of commands",
                instruction_pointer
            ),
        }

        instruction_pointer += 1
    }

    output
}

fn main() {
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let output = brainfuck(hello_world);

    for o in &output {
        match String::from_utf8(vec![*o]) {
            Ok(output_string) => print!("{}", output_string),
            Err(_) => (),
        };
    }
}
