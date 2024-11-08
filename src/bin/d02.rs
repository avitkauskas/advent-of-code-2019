use std::fs;

fn main() {
    // Read input file
    let input = fs::read_to_string("../../inputs/d02.txt").expect("Failed to read input file");

    // Parse input into vector of numbers
    let mut program: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Restore the "1202 program alarm" state
    program[1] = 12;
    program[2] = 2;

    // Run the program
    let result = run_intcode(program);
    println!("Value at position 0: {}", result);
}

fn run_intcode(mut program: Vec<usize>) -> usize {
    let mut position = 0;

    while position < program.len() {
        match program[position] {
            99 => break,
            1 => {
                let pos1 = program[position + 1];
                let pos2 = program[position + 2];
                let output_pos = program[position + 3];
                program[output_pos] = program[pos1] + program[pos2];
            }
            2 => {
                let pos1 = program[position + 1];
                let pos2 = program[position + 2];
                let output_pos = program[position + 3];
                program[output_pos] = program[pos1] * program[pos2];
            }
            _ => panic!("Unknown opcode encountered!"),
        }
        position += 4;
    }

    program[0]
}
