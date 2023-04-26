#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf
)]

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::Read;
use std::usize;

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sequence of commands supplied to the interpreter
    input: String,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn brainfuck(data: &str) -> Result<Vec<u8>, Error> {
    const CELL_SIZE: usize = 30000;
    let commands: Vec<u8> = data.as_bytes().to_vec();
    let mut instruction_pointer: usize = 0;

    let mut cells = [0u8; CELL_SIZE];
    let mut data_pointer: usize = 0;
    let mut stack: Vec<usize> = Vec::new();

    let mut input = [0u8; 1];
    let mut output: Vec<u8> = Vec::new();

    while let Some(instruction) = commands.get(instruction_pointer) {
        match instruction {
            b'>' => data_pointer += 1,
            b'<' => data_pointer -= 1,
            b'+' => match cells.get_mut(data_pointer) {
                Some(cell) => *cell = cell.wrapping_add(1),
                None => return Err(Error::PointerOutsideOfCells(data_pointer, CELL_SIZE)),
            },
            b'-' => match cells.get_mut(data_pointer) {
                Some(cell) => *cell = cell.wrapping_sub(1),
                None => return Err(Error::PointerOutsideOfCells(data_pointer, CELL_SIZE)),
            },
            b'[' => match cells.get(data_pointer) {
                Some(0) => instruction_pointer = bracket_match(instruction_pointer, &commands)?,
                None => return Err(Error::PointerOutsideOfCells(data_pointer, CELL_SIZE)),
                _ => stack.push(instruction_pointer),
            },
            b']' => match cells.get(data_pointer) {
                Some(0) => _ = stack.pop(),
                None => return Err(Error::PointerOutsideOfCells(data_pointer, CELL_SIZE)),
                _ => match stack.last() {
                    Some(next_pointer) => instruction_pointer = *next_pointer,
                    None => return Err(Error::NoMatchingLHSBracket),
                },
            },
            b',' => match io::stdin().read_exact(&mut input) {
                Ok(_) => cells[data_pointer] = input[0],
                Err(_) => return Err(Error::FailedInput),
            },
            b'.' => match cells.get(data_pointer) {
                Some(cell_content) => output.push(*cell_content),
                _ => return Err(Error::PointerOutsideOfCells(data_pointer, CELL_SIZE)),
            },
            _ => (),
        };

        instruction_pointer += 1;
    }

    Ok(output)
}

fn bracket_match(instruction_pointer: usize, commands: &[u8]) -> Result<usize, Error> {
    let mut depth = 1;
    let mut instruction_pointer = instruction_pointer;

    while depth > 0 {
        instruction_pointer += 1;
        match commands.get(instruction_pointer) {
            Some(instruction) => match instruction {
                b'[' => depth += 1,
                b']' => depth -= 1,
                _ => (),
            },
            None => return Err(Error::NoMatchingRHSBracket),
        };
    }

    Ok(instruction_pointer)
}

#[derive(Debug)]
enum Error {
    PointerOutsideOfCells(usize, usize),
    NoMatchingRHSBracket,
    NoMatchingLHSBracket,
    FailedInput,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PointerOutsideOfCells(u, s) => {
                write!(
                    f,
                    "data pointer at index `{u}` moved outside of cells with size {s}"
                )
            }
            Self::NoMatchingRHSBracket => f.write_str("no `[` in character range"),
            Self::NoMatchingLHSBracket => f.write_str("no `]` in character range"),
            Self::FailedInput => f.write_str("failed to read input"),
        }
    }
}

fn main() {
    let args = Cli::parse();
    let result = brainfuck(&args.input);

    match result {
        Ok(output) => {
            for o in &output {
                if let Ok(output_string) = String::from_utf8(vec![*o]) {
                    print!("{output_string}");
                };
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
