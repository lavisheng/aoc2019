use std::collections::HashSet;
#[aoc_generator(day4)]
pub fn range_gen(input: &str) -> [i32;2]{
    let lower = input[0..6].parse::<i32>().unwrap();
    let upper = input[7..13].parse::<i32>().unwrap();
    [lower, upper]
}
pub fn validity(num: i32)->bool{
    // iterator and memory of last 
    let string = num.to_string();
    let mut iterator = string.chars().into_iter();
    let mut last = iterator.next().unwrap();
    // flags
    let mut double = false;
    let mut non_decr = true;
    for curr in iterator{
        if curr == last{
            double = true;
        }
        if curr.to_digit(10).unwrap() <
            last.to_digit(10).unwrap(){
                non_decr = false;
            }
        last = curr;
    }
    double && non_decr
}
#[aoc(day4, part1)]
pub fn day4_pt1(range: &[i32])-> i32 {
    let mut counter = 0;
    for num in range[0]..range[1]{
        if validity(num){
            counter += 1;
        }
    }
    counter
}

pub fn validity_pt2(num: i32) -> bool{
    // iterator and memory of last 
    let string = num.to_string();
    let mut iterator = string.chars().into_iter();
    let mut last = iterator.next().unwrap();
    // flags
    let mut double = false;
    let mut non_decr = true;
    let mut matched = 0;
    let mut group_flag = false;
    for curr in iterator{
        if curr == last{
            double = true;
            matched += 1;
        } else {
            if matched == 1{
                group_flag = true;
            }
            matched = 0;
        }
        if curr.to_digit(10).unwrap() <
            last.to_digit(10).unwrap(){
                non_decr = false;
        }
        last = curr;
    }
    if matched == 1 { group_flag = true; }
    double && non_decr && group_flag

}
#[aoc(day4, part2)]
pub fn day4_pt2(range: &[i32]) -> i32{
    let mut counter = 0;
    for num in range[0]..range[1]{
        if validity_pt2(num){
            counter += 1;
        }
    }
    counter
}
