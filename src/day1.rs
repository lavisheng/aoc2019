#[aoc(day1, part1)]
pub fn day1(input: &str) -> String {
    let mut total: i32 = 0;
    for line in input.lines(){
        total += line.parse::<i32>().unwrap() / 3 - 2;
    }
    total.to_string()
}

pub fn recursive_fuel_helper(fuel: i32) -> i32{
    let c = fuel/3 - 2;
    if c <= 0{
        return 0;
    } else {
        return c + recursive_fuel_helper(c);
    }
}
#[aoc(day1, part2)]
pub fn day2(input: &str) -> String {
    let mut total: i32 = 0;
    for line in input.lines(){
        //total += lines.parse()::<i32>() /3 -2;
        total += recursive_fuel_helper(line.parse::<i32>().unwrap());
    }
    total.to_string()
}
