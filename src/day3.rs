use std::collections::{HashSet, HashMap};
// honestly overly complicated
pub struct Line{
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    x_orientation: i32,
    y_orientation: i32,
    length: i32,
}
impl Line{
    fn new(s_x: i32, s_y: i32, e_x: i32, e_y: i32)->Line{
        let out = Line{
            start_x: s_x,
            start_y: s_y,
            end_x: e_x,
            end_y: e_y,
            x_orientation: {
                if e_x == s_x { 0 }
                else { (e_x - s_x)/ (e_x - s_x).abs()}
                },
            y_orientation: {
                if e_y == s_y { 0 }
                else { (e_y - s_y)/ (e_y - s_y).abs()}
            },
            length: (e_y - s_y).abs() + (e_x - s_x).abs(),
        };
        out
    }
    // expands into all points the line covers
    fn expand(&self)->Vec<(i32, i32)>{
        let mut out: Vec<(i32, i32)> = Vec::new();
        let mut i_x = self.start_x.clone();
        let mut i_y = self.start_y.clone();
        // while loop to expand the line
        while i_x != self.end_x || i_y != self.end_y{
            out.push((i_x, i_y));
            i_x += self.x_orientation;
            i_y += self.y_orientation;
        }
        out.push((self.end_x, self.end_y));
        return out;
    }

    // Expands into all points the line covers, and 3rd is distance up to point
    fn expand_with_d(&self, d: i32) -> Vec<(i32, i32, i32)>{
        let mut out: Vec<(i32, i32, i32)> = Vec::new();
        let mut i_x = self.start_x.clone();
        let mut i_y = self.start_y.clone();
        let mut steps = 0;
        // expand with distance to get there
        while i_x != self.end_x || i_y != self.end_y{
            out.push((i_x, i_y, steps + d));
            i_x += self.x_orientation;
            i_y += self.y_orientation;
            steps += 1;
        }
        out.push((self.end_x, self.end_y, steps));
        return out;
    }
}
pub struct Coords{
    x: i32,
    y: i32,
}

impl Coords{
    fn new() -> Coords{
        Coords{
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self, delta: (i32, i32)) -> Line{
        let initx = self.x;
        let inity = self.y;
        self.x += delta.0;
        self.y += delta.1;
        Line::new(initx, inity, self.x, self.y)
    }
}


#[aoc_generator(day3)]
pub fn wires_gen(input: &str) -> Vec<Vec<(i32, i32)>>{
    let lines: Vec<&str> = input.lines().collect();
    let mut out: Vec<Vec<(i32, i32)>> = Vec::new();
    
    for line in input.lines(){
        let mut deltas: Vec<(i32, i32)> = Vec::new();
        for d in line.trim().split(","){
            // convert based off of RLUD for direction
            let delta = match d.chars().nth(0).unwrap(){
                'R' => (d[1..].parse::<i32>().unwrap(), 0),
                'L' => (-1 * d[1..].parse::<i32>().unwrap(),0),
                'U' => (0, d[1..].parse::<i32>().unwrap()),
                'D' => (0, -1 * d[1..].parse::<i32>().unwrap()),
                _ => (0, 0),
            };
            deltas.push(delta);
        }
        out.push(deltas);
    }
    out
}

#[aoc(day3, part1)]
pub fn day3_pt1(deltas: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut p1_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut p2_coords: HashSet<(i32, i32)> = HashSet::new();

    // assume the vector only has two in it
    let mut p1 = Coords::new();
    let mut p2 = Coords::new();
    //step through the first one
    for d in &deltas[0]{
        // get all the points in the first wiring
        let line = p1.step(*d);
        let values = line.expand();
        for i in values.iter(){
            p1_coords.insert(*i);
        }
    }
    for d in &deltas[1]{
        // get all poitns in second wiring
        let line = p2.step(*d);
        for i in line.expand().iter(){
            p2_coords.insert(*i);
        }
    }
    // find minimum in the intersection
    let mut d_min: i32 = -1;
    for (x, y) in p1_coords.intersection(&p2_coords){
        if *x + *y != 0 && (x.abs() + y.abs() < d_min || d_min == -1){
            //println!("{}, {}", x, y);
            d_min = x.abs() + y.abs();
        }
    }
    d_min
}

#[aoc(day3, part2)]
pub fn day3_pt2(deltas: &Vec<Vec<(i32, i32)>>) -> i32{
    // modified version of the first one where we have a distance measurement
    let mut intersections: HashSet<(i32, i32)> = HashSet::new();
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    // setup variables
    let mut p1 = Coords::new();
    let mut p1_d = 0;
    let mut p2 = Coords::new();
    let mut p2_d = 0;
    // initialize points with everything in the first wire
    for d in &deltas[0]{
        let line = p1.step(*d);
        for (x, y, d) in line.expand_with_d(p1_d).iter(){
            points.entry((*x, *y)).or_insert(*d);
        }
        p1_d += line.length;
    }

    // Only check where the second wire intersects with first wire
    // since distance increases if we encounter the same intersection another
    // time we ignore it
    for d in &deltas[1]{
        let line = p2.step(*d);
        for(x, y, d) in line.expand_with_d(p2_d).iter(){
            if points.contains_key(&(*x,*y)) &&
                !intersections.contains(&(*x,*y)){
                let mut distance = points.get_mut(&(*x,*y)).unwrap();
                *distance += d;
                intersections.insert((*x,*y));
            }
        }
        p2_d += line.length;
    }

    // find minimum non 0 distance
    // note that (0, 0) is included amongst intersections
    let mut d_min = -1;
    for coord in intersections.iter(){
        if d_min == -1 ||
            (*points.get(coord).unwrap() != 0 &&
             *points.get(coord).unwrap() < d_min){
            d_min = *points.get(coord).unwrap();
        }
    }
    d_min
}
