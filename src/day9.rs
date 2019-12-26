use std::collections::{VecDeque, HashMap};
#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<i64>{
    input.trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}
fn opcode(val: &i64) -> [i64;4]{
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
pub struct Machine{
    tape: Vec<i64>,
    state_ptr: usize,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    finished: bool,
    last_out: i64,
    relative: i64,
    // store additional memory in a map
    additional: HashMap<usize, i64>,
}

impl Machine{
    fn new(tape: &Vec<i64>) -> Machine{
        Machine{
            tape: tape.clone(),
            state_ptr: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            finished: false,
            last_out: -1,
            relative: 0,
            additional: HashMap::new()
        }
    }

    fn write(&mut self, location: usize, val: i64){
        if location >= self.tape.len(){
            let addr = self.additional.entry(location).or_insert(0);
            *addr = val;
        } else {
            self.tape[location] = val;
        }
    }

    fn read(&self, location: usize) -> i64{
        if location >= self.tape.len(){
            match self.additional.get(&(location as usize)){
                Some(&v) => return v,
                None => return 0,
            }
        }
        self.tape[location]
    }

    fn read_delta(&self, delta: usize) -> i64{
        self.read(delta + self.state_ptr)
    }

    fn get_value(&self, val: i64, mode: i64, tape: &Vec<i64>) -> i64 {
        match mode {
            0 => {
                return self.read(val as usize);
            },
            1 => {
                return val;
            },
            2 => {
                return self.read((self.relative + val) as usize);
            },
            _ => {
                println!("ERROR");
                return 0;
            }
        }
    }

    fn get_dest(&self, val: i64, mode: i64, tape: &Vec<i64>) -> usize{
        match mode{
            0 => {
                return val as usize;
            },
            2 => {
                return (self.relative + val) as usize;
            },
            _ => {
                println!("ERROR, got in dest: {}", mode);
                return 0;
            }
        }
    }
    fn step(&mut self){
        let code = opcode(&self.read(self.state_ptr));
        match code[0]{
            1 => {
                let dest = self.get_dest(self.read_delta(3), code[3], &self.tape) as usize;
                self.write(dest,
                           self.get_value(self.read_delta(1), code[1], &self.tape) +
                           self.get_value(self.read_delta(2), code[2], &self.tape));
                self.state_ptr += 4;
            },
            2=> {
                let dest = self.get_dest(self.read_delta(3), code[3], &self.tape);
                self.write(dest,
                           self.get_value(self.read_delta(1), code[1], &self.tape) *
                           self.get_value(self.read_delta(2), code[2], &self.tape));
                self.state_ptr += 4;
            },
            3 => {
                let dest = self.get_dest(self.read_delta(1), code[1], &self.tape);
                match self.inputs.pop_front(){
                    Some(x) => {
                        self.write(dest, x);
                        self.state_ptr += 2;
                    },
                    None => return,
                }
            },
            4 => {
                let out = self.get_value(self.read_delta(1), code[1], &self.tape);
                self.outputs.push_back(out);
                self.last_out = out;
                self.state_ptr += 2;
            },
            5 => {
                if self.get_value(self.read_delta(1), code[1], &self.tape) != 0{
                    self.state_ptr = self.get_value(
                        self.read_delta(2),
                        code[2],
                        &self.tape) as usize;
                } else {
                    self.state_ptr += 3;
                }
            },
            6 => {
                if self.get_value(self.read_delta(1), code[1], &self.tape) == 0{
                    self.state_ptr = self.get_value(
                        self.read_delta(2),
                        code[2],
                        &self.tape) as usize;
                } else {
                    self.state_ptr += 3;
                }
            },
            7 => {
                let dest = self.get_dest(self.read_delta(3), code[3], &self.tape);
                if self.get_value(self.read_delta(1), code[1], &self.tape) <
                    self.get_value(self.read_delta(2), code[2], &self.tape){
                        self.write(dest, 1);
                    } else {
                        self.write(dest, 0);
                    }
                self.state_ptr += 4;
            },
            8 => {
                let dest = self.get_dest(self.read_delta(3), code[3], &self.tape);
                if self.get_value(self.read_delta(1), code[1], &self.tape) ==
                    self.get_value(self.read_delta(2), code[2], &self.tape){
                        self.write(dest, 1);
                    } else {
                        self.write(dest, 0);
                    }
                self.state_ptr += 4;
            },
            9 => {
                self.relative += self.get_value(self.read_delta(1), code[1], &self.tape);
                self.state_ptr += 2;
            },
            99 => self.finished = true,
            _ => {
                println!("Code: {}", code[0]);
                print!("ERROR, bad opcode\n");
            }
        }
    }
}

#[aoc(day9, part1)]
pub fn day9_pt1(tape: &Vec<i64>) -> i64{
    let mut m1 = Machine::new(&tape);
    m1.inputs.push_back(1);
    while !m1.finished{
        m1.step();
    }
    m1.outputs.pop_front().unwrap()
} 

#[aoc(day9, part2)]
pub fn day9_pt2(tape: &Vec<i64>) -> i64{
    let mut m1 = Machine::new(&tape);
    m1.inputs.push_back(2);
    while !m1.finished{
        m1.step();
    }
    m1.outputs.pop_front().unwrap()
}
