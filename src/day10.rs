use std::collections::{HashSet, HashMap};
#[aoc_generator(day10)]
pub fn input_gen(input: &str) -> Vec<(i32, i32)>{
    input.lines()
        .enumerate()
        .map(|(y, v)| v.trim()
             .chars()
             .enumerate()
             .filter(|(x, c)| *c == '#')
             .map(|(x, _)| (x as i32,y as i32))
             .collect::<Vec<(i32, i32)>>())
        .flatten()
        .collect::<Vec<(i32, i32)>>()
}

pub fn gcd(num1: usize, num2: usize) -> i32{
    if num2 == 0{
        return num1 as i32;
    }
    gcd(num2, num1 % num2) as i32
}

pub fn simplify(num1: i32, num2: i32) -> (i32, i32){
    // assume that both cannot be 0 (valid in this case)
    let divisor = gcd(num1.abs() as usize, num2.abs() as usize);
    (num1 / divisor, num2 / divisor)
}

pub fn get_station_coord(coordinates: &[(i32, i32)]) -> (i32, i32, i32){
    let mut max: i32 = -1;
    let mut station_loc = (-1, -1, -1);
    for coord in coordinates{
        // for each asteroid, ignore own coordinate
        // check others and so long as they aren't along a line
        // (aka, the ratio between x and y is unique)
        // then this can be seen
        // otherwise no line of sight
        let asteroids = coordinates.iter()
            .filter(|v| *v != coord)
            .map(|(x, y)| (x - coord.0, y - coord.1))
            .map(|(x, y)| simplify(x, y))
            .collect::<HashSet<(i32, i32)>>()
            .len();
        if station_loc.2 == -1 || asteroids as i32 > station_loc.2 {
            station_loc = (coord.0, coord.1, asteroids as i32);
        }
    }
    station_loc
}
#[aoc(day10, part1)]
pub fn day10_pt1(coordinates: &[(i32, i32)]) -> i32{
    get_station_coord(coordinates).2
}

// explicitly calculate the angle from up angle
// for sorting purposes
pub fn get_angle(x: i32, y: i32) -> f64{
    if x == 0{
        if y > 0{
            return 0f64;
        } else {
            return std::f64::consts::PI;
        }
    } else if x > 0{
        return std::f64::consts::PI / 2f64 - (y as f64 / x as f64).atan();
    } else {
        return std::f64::consts::PI * 1.5 + (y as f64 / x as f64).atan();
    }
}
pub fn norm(x: i32, y: i32) -> i32{
    x.abs() + y.abs()
}
#[aoc(day10, part2)]
pub fn day10_pt2(coordinates: &[(i32, i32)]) -> i32{
    let location = get_station_coord(coordinates);
    // creates a vector where we calculate the relative coordinate from the location
    // and last two are original coordinates so we can calculate the answer
    let mut other_coords = coordinates.into_iter()
        .filter(|&&v| v != (location.0, location.1))
        .map(|(x, y)| (x - location.0, y - location.1))
        .collect::<Vec<(i32, i32)>>();
    // collect the deltas only
    let mut to_remove: i32 = 200;
    while to_remove > 0 {
        let mut ring_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        other_coords.iter()
            .for_each(|(dx, dy)| {
                let angle = simplify(*dx, *dy);
                let coord = ring_map.entry(angle).or_insert((*dx, *dy));
                if norm(*dx, *dy) < norm(coord.0, coord.1){
                    *coord = (*dx, *dy);
                }
            });
        let mut outer_asteroids = ring_map.values().map(|v| *v).collect::<Vec<(i32, i32)>>();
        if outer_asteroids.len() > to_remove as usize{
            // the important one is in this ring
            outer_asteroids.sort_by(|(ax, ay), (bx, by)| get_angle(*ax, *ay)
                                    .partial_cmp(&get_angle(*bx, *by)).unwrap());
            let two_hundredth = outer_asteroids.iter()
                .nth((to_remove - 1) as usize).unwrap();
            return (location.0 + two_hundredth.0) * 100 + location. 1 + two_hundredth.1;
        } else {
            // not in this ring, remove
            other_coords = other_coords.iter()
                .filter(|v| outer_asteroids.contains(v))
                .map(|v| *v)
                .collect();
            to_remove -= outer_asteroids.len() as i32;
        }
    }
    // should never reach here
    return -1
}
