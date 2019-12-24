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
pub fn run_tape_pt2(tape: &mut Vec<i32>) -> i32{
    let mut i = 0;
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
                let mut buf = String::new();
                io::stdin().read_line(&mut buf)
                    .expect("Failed to read");
                let dest = tape[i + 1] as usize;
                tape[dest] = buf.trim().parse::<i32>().unwrap();
                i += 2;
            },
            4 => {
                let out = get_value(tape[i + 1], code[1], tape);
                println!("opcode 4 out: {}", out);
                i+= 2;
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

