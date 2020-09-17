use std::collections::HashSet;
use crate::machine::Machine;
#[aoc_generator(day13)]
pub fn input_gen(input: &str) -> Vec<i64>{
    input.trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day13, part1)]
pub fn day11_pt1(tape: &Vec<i64>)-> usize{
    let mut machine = Machine::new(tape);
    let mut buf:[i64;3] = [0; 3];
    let mut buf_pt = 0;
    let mut points: HashSet<(i64, i64)> = HashSet::new();
    while !machine.finished{
        machine.step();
        match machine.get_output(){
            Some(x) => {
                buf[buf_pt] = x;
                buf_pt += 1;
            },
            None => (),
        }
        if buf_pt == 3{
            // reset
            buf_pt = 0;
            // insert point if block
            if buf[2] == 2{
                points.insert((buf[0], buf[1]));
            }
        }
    }
    points.len()
}

#[aoc(day13, part2)]
pub fn day11_pt2(tape: &Vec<i64>) -> usize {
    let mut machine = Machine::new(tape);
    // modify the tape before running
    machine.write(0, 2);
    // dummy inputs to ensure program doesn't hang
    machine.inputs.push_back(0);
    machine.inputs.push_back(0);

    let mut buf: [i64; 3] = [0;3];
    let mut buf_pt = 0;

    let mut walls: Vec<(usize, usize)> = Vec::new();
    let mut (ballx, bally) = (0, 0);
    let mut score = 0;
    // on first iteration up until the score, we check
    // Strategy: Find write location of every block, and then modify the tape
    // such that their x and y redirect to the ball's x and y.
    let mut first = true;
    while !machine.finished{
        machine.step();
        match machine.get_output(){
            Some(x) => {
                buf[buf_pt] = x;
                buf_pt += 1;
            },
            None => (),
        }
        if buf_pt == 3{
            buf_pt = 0;
            println!("type: {}", buf[2]);
            // score! end of a single gameplay loop:
            if first {
                let y = machine.get_print_loc().unwrap();
                let x = machine.get_print_loc().unwrap();
                let _ = machine.get_print_loc().unwrap();
                if buf[2] == 4{
                    // ball!
                    ballx = x;
                    bally = y;
                } else if buf[2] == 2{
                    // wall
                    walls.push_back((x,y));
                }
            }
            }
            if buf[0] == -1 && buf[1] == 0{
                score = buf[2];
                if first{
                    first = false;
                    walls.iter()
                        .foreach(|(x, y)|{
                            
                        });
                }
            }
        }
    buf_pt
}
