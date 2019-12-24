use permutohedron::heap_recursive;
use std::collections::VecDeque;
use std::iter::FromIterator;
#[aoc_generator(day7)]
pub fn input_gen(input: &str) -> Vec<i32>{
    input.trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn opcode(val: &i32) -> [i32;4]{
    let mut code = val.clone();
    // get the first 2 digits
    let op = code % 100;
    code = code / 100;
    // get modes
    let p1_mode = code % 10;
    code = code / 10;
    let p2_mode = code % 10;
    code =  code / 10;

    let p3_mode = code % 10;
    [op, p1_mode, p2_mode, p3_mode]
}
pub fn get_value(val: i32, mode: i32, tape: &Vec<i32>) -> i32 {
    match mode {
        0 => {
            return tape[val as usize];
        },
        1 => {
            return val;
        },
        _ => {
            println!("ERROR");
            return 0;
        }
    }
}

pub fn run_tape(tape: &mut Vec<i32>, input: Vec<i32>) -> i32{
    let mut i = 0;
    let mut input_pt = 0;
    loop {
        let code = opcode(&tape[i]);
        match code[0]{
            1 => {
                let dest = tape[i + 3] as usize;
                tape[dest] = get_value(tape[i + 1], code[1], tape) +
                    get_value(tape[i + 2], code[2], tape);
                i += 4;
            },
            2=> {
                let dest = tape[i + 3] as usize;
                tape[dest] = get_value(tape[i + 1], code[1], tape) *
                    get_value(tape[i + 2], code[2], tape);
                i += 4;
            },
            3 => {
                let dest = tape[i + 1] as usize;
                tape[dest] = input[input_pt];
                input_pt += 1; 
                i += 2;
            },
            4 => {
                let output = get_value(tape[i + 1], code[1], tape);
                return output;
            },
            5 => {
                if get_value(tape[i + 1], code[1], tape) != 0{
                    i = get_value(tape[i + 2], code[2], tape) as usize;
                } else {
                    i += 3;
                }
            },
            6 => {
                if get_value(tape[i + 1], code[1], tape) == 0{
                    i = get_value(tape[i + 2], code[2], tape) as usize;
                } else {
                    i += 3;
                }
            },
            7 => {
                //let dest = get_value(tape[i + 3], code[3], tape) as usize; 
                let dest = tape[i + 3] as usize;
                if get_value(tape[i + 1], code[1], tape) <
                    get_value(tape[i + 2], code[2], tape){
                    tape[dest] = 1;
                } else {
                    tape[dest] = 0;
                }
                i += 4;
            },
            8 => {
                let dest = tape[i + 3] as usize;
                //let dest = get_value(tape[i + 3], code[3], tape) as usize; 
                if get_value(tape[i + 1], code[1], tape) ==
                    get_value(tape[i + 2], code[2], tape){
                    tape[dest] = 1;
                } else {
                    tape[dest] = 0;
                }
                i += 4;
            },
            99 => break,
            _ => {
                println!("Code: {}", code[0]);
                print!("ERROR, bad opcode\n");
                return 0
            }
        }
    }
    tape[0]
}

#[aoc(day7, part1)]
pub fn day7_pt1(tape: &Vec<i32>) -> i32 {
    let mut input_alphabet = [0, 1, 2, 3, 4];
    let mut max = -1;

    heap_recursive(&mut input_alphabet, |perm|{
        //let mut states = tape.clone();
        let mut a_states = tape.clone();
        let a_sig = run_tape(&mut a_states, vec![perm[0], 0]);
        let mut b_states = tape.clone();
        let b_sig = run_tape(&mut b_states, vec![perm[1], a_sig]);
        let mut c_states = tape.clone();
        let c_sig = run_tape(&mut c_states, vec![perm[2], b_sig]);
        let mut d_states = tape.clone();
        let d_sig = run_tape(&mut d_states, vec![perm[3], c_sig]);
        let mut e_states = tape.clone();
        let e_sig = run_tape(&mut e_states, vec![perm[4], d_sig]);
        if max == -1 || e_sig > max {
            max = e_sig;
        }
    });
    max
}

// only for chained
pub struct Machine{
    tape: Vec<i32>,
    state_ptr: usize,
    inputs:  VecDeque<i32>,
    outputs: VecDeque<i32>,
    finished: bool,
    last_out: i32,
}


impl Machine{
    fn new(tape: Vec<i32>, p: i32) -> Machine{
        let mut states = tape.clone();
        let mut out = Machine{
            tape: states,
            state_ptr: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            finished: false,
            last_out: -1,
        };
        out.inputs.push_back(p);
        out
    }

    fn step(&mut self){
        let code = opcode(&self.tape[self.state_ptr]);
        match code[0]{
            1 => {
                let dest = self.tape[self.state_ptr + 3] as usize;
                self.tape[dest] = get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) +
                    get_value(self.tape[self.state_ptr + 2], code[2], &self.tape);
                self.state_ptr += 4;
            },
            2=> {
                let dest = self.tape[self.state_ptr + 3] as usize;
                self.tape[dest] = get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) *
                    get_value(self.tape[self.state_ptr + 2], code[2], &self.tape);
                self.state_ptr += 4;
            },
            3 => {
                let dest = self.tape[self.state_ptr + 1] as usize;
                match self.inputs.pop_front(){
                    Some(x) => {
                        self.tape[dest] = x;
                        self.state_ptr += 2;
                    },
                    None => return,
                }
            },
            4 => {
                let out = get_value(self.tape[self.state_ptr + 1], code[1], &self.tape);
                //println!("Adding to outputs!, {}", out);
                self.outputs.push_back(out);
                self.last_out = out;
                self.state_ptr += 2;
            },
            5 => {
                if get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) != 0{
                    self.state_ptr = get_value(self.tape[self.state_ptr + 2], code[2], &self.tape) as usize;
                } else {
                    self.state_ptr += 3;
                }
            },
            6 => {
                if get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) == 0{
                    self.state_ptr = get_value(
                        self.tape[self.state_ptr + 2],
                        code[2],
                        &self.tape) as usize;
                } else {
                    self.state_ptr += 3;
                }
            },
            7 => {
                //let dest = get_value(self.tape[i + 3], code[3], self.tape) as usize; 
                let dest = self.tape[self.state_ptr + 3] as usize;
                if get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) <
                    get_value(self.tape[self.state_ptr + 2], code[2], &self.tape){
                    self.tape[dest] = 1;
                } else {
                    self.tape[dest] = 0;
                }
                self.state_ptr += 4;
            },
            8 => {
                let dest = self.tape[self.state_ptr + 3] as usize;
                //let dest = get_value(self.tape[i + 3], code[3], self.tape) as usize; 
                if get_value(self.tape[self.state_ptr + 1], code[1], &self.tape) ==
                    get_value(self.tape[self.state_ptr + 2], code[2], &self.tape){
                    self.tape[dest] = 1;
                } else {
                    self.tape[dest] = 0;
                }
                self.state_ptr += 4;
            },
            99 => self.finished = true,
            _ => {
                println!("Code: {}", code[0]);
                print!("ERROR, bad opcode\n");
            }
        }
    }
}

#[aoc(day7, part2)]
pub fn day7_pt2(tape: &Vec<i32>) -> i32 {
    let mut input_alphabet = [5,6,7,8,9];
    let mut max = -1;

    heap_recursive(&mut input_alphabet, |perm|{
        let mut a_states = tape.clone();
        let mut a = Machine::new(a_states, perm[0]);
        let b_states = tape.clone();
        let mut b = Machine::new(b_states, perm[1]);
        let c_states = tape.clone();
        let mut c = Machine::new(c_states, perm[2]);
        let d_states = tape.clone();
        let mut d = Machine::new(d_states, perm[3]);
        let e_states = tape.clone();
        let mut e = Machine::new(e_states, perm[4]);
        a.inputs.push_back(0);
        while !e.finished {
            // chain the inputs and outputs
            a.inputs.append(&mut e.outputs);
            a.step();
            b.inputs.append(&mut a.outputs);
            b.step();
            c.inputs.append(&mut b.outputs);
            c.step();
            d.inputs.append(&mut c.outputs);
            d.step();
            e.inputs.append(&mut d.outputs);
            e.step();
        }
        let sig = e.last_out;
        if max == -1 || sig > max {
            max = sig;
        }
    });
    max
}
