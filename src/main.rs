#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf
)]

use std::io;
use std::io::Read;
use std::usize;

enum BrainfuckError {
    PointerOutsideOfCells,
    NoMatchingRHSBracket,
    NoMatchingLHSBracket,
    FailedInput,
}

fn brainfuck(data: &str) -> Result<Vec<u8>, BrainfuckError> {
    const CELL_SIZE: usize = 30000;
    let commands: Vec<u8> = data.as_bytes().to_vec();
    let mut instruction_pointer = 0;

    let mut cells = [0u8; CELL_SIZE];
    let mut data_pointer = 0;
    let mut stack: Vec<usize> = Vec::new();

    let mut input = [0u8; 1];
    let mut output: Vec<u8> = Vec::new();

    while let Some(instruction) = commands.get(instruction_pointer) {
        let iteration = match instruction {
            b'>' => {
                data_pointer += 1;
                Ok(())
            }
            b'<' => {
                data_pointer -= 1;
                Ok(())
            }
            b'+' => match cells.get_mut(data_pointer) {
                Some(cell) => {
                    *cell = cell.wrapping_add(1);
                    Ok(())
                }
                _ => Err(BrainfuckError::PointerOutsideOfCells),
            },
            b'-' => match cells.get_mut(data_pointer) {
                Some(cell) => {
                    *cell = cell.wrapping_sub(1);
                    Ok(())
                }
                _ => Err(BrainfuckError::PointerOutsideOfCells),
            },
            b'[' => match cells.get(data_pointer) {
                Some(0) => {
                    instruction_pointer = bracket_match(instruction_pointer, &commands)?;
                    Ok(())
                }
                None => Err(BrainfuckError::PointerOutsideOfCells),
                _ => {
                    stack.push(instruction_pointer);
                    Ok(())
                }
            },
            b']' => match cells.get(data_pointer) {
                Some(0) => {
                    _ = stack.pop();
                    Ok(())
                }
                None => Err(BrainfuckError::PointerOutsideOfCells),
                _ => match stack.last() {
                    Some(next_pointer) => {
                        instruction_pointer = *next_pointer;
                        Ok(())
                    }
                    None => Err(BrainfuckError::NoMatchingLHSBracket),
                },
            },
            b',' => match io::stdin().read_exact(&mut input) {
                Ok(_) => {
                    cells[data_pointer] = input[0];
                    Ok(())
                }
                Err(_) => Err(BrainfuckError::FailedInput),
            },
            b'.' => match cells.get(data_pointer) {
                Some(cell_content) => {
                    output.push(*cell_content);
                    Ok(())
                }
                _ => Err(BrainfuckError::PointerOutsideOfCells),
            },
            _ => Ok(()),
        };

        match iteration {
            Ok(_) => instruction_pointer += 1,
            Err(error) => return Err(error),
        }
    }

    Ok(output)
}

fn bracket_match(instruction_pointer: usize, commands: &[u8]) -> Result<usize, BrainfuckError> {
    let mut depth = 1;
    let mut instruction_pointer = instruction_pointer;

    while depth > 0 {
        instruction_pointer += 1;
        match commands.get(instruction_pointer) {
            Some(instruction) => match instruction {
                b'[' => {
                    depth += 1;
                }
                b']' => {
                    depth -= 1;
                }
                _ => (),
            },
            None => {
                return Err(BrainfuckError::NoMatchingRHSBracket);
            }
        };
    }

    Ok(instruction_pointer)
}

fn main() {
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    if let Ok(output) = brainfuck(hello_world) {
        for o in &output {
            if let Ok(output_string) = String::from_utf8(vec![*o]) {
                print!("{output_string}");
            };
        }
    }
}
