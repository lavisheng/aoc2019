#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<usize>{
    input.trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn run_tape(mut tape: Vec<usize>) -> usize{
    let mut i = 0;
    loop {
        match tape[i]{
            1 => {
                let dest = tape[i+3];
                tape[dest] = tape[tape[i+1]] + tape[tape[i+2]]
            },
            2=> {
                let dest = tape[i+3];
                tape[dest] = tape[tape[i+1]] * tape[tape[i+2]]
            },
            99 => break,
            _ => {
                print!("ERROR, bad opcode\n");
                return 0
            }
        }
        i += 4;
    }
    tape[0]
}
#[aoc(day2, part1)]
pub fn day2(tape: &Vec<usize>) -> String {
    let mut tape = tape.clone();
    tape[1] = 12;
    tape[2] = 2;
    run_tape(tape).to_string()
}

#[aoc(day2, part2)]
pub fn day2_part2(tape: &Vec<usize>) -> String {
    for noun in 1..99{
        for verb in 1..99{
            let mut t = tape.clone();
            t[1] = noun;
            t[2] = verb;
            if run_tape(t) == 19690720 {
                return format!("{}", 100 * noun + verb);
            }
        }
    }
    "ERROR".to_string()
}
