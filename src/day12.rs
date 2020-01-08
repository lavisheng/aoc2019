#[derive(Copy, Clone)]
pub struct Moon{
    x: i32,
    y: i32,
    z: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}
impl Moon{
    fn new(x: i32, y: i32, z: i32) -> Moon{
        Moon{
            x: x,
            y: y,
            z: z,
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }

    // Apply gravity to the moons to determine how gravity acts upon them
    fn gravity(&mut self, moons: &[(i32, i32, i32)]){
        for m in moons{
            if m.0 > self.x{
                self.dx += 1;
            } else if m.0 < self.x{
                self.dx -= 1;
            }

            if m.1 > self.y{
                self.dy += 1;
            } else if m.1 < self.y{
                self.dy -= 1;
            }

            if m.2 > self.z{
                self.dz += 1;
            } else if m.2 < self.z{
                self.dz -= 1;
            }
        }
    }

    fn step(&mut self){
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    fn pot_energy(&self) -> i32{
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    fn kin_energy(&self) -> i32{
        self.dx.abs() + self.dy.abs() + self.dz.abs()
    }

    pub fn values(&self) -> (i32, i32, i32){
        (self.x, self.y, self.z)
    }

}

#[aoc_generator(day12)]
pub fn input_gen(input: &str) -> Vec<Moon>{
    let mut moons: Vec<Moon> = Vec::new();
    for line in input.trim().lines(){
        let data = line.split(|x| x == '=' || x == ',' || x == '>')
            .collect::<Vec<&str>>();
        moons.push(Moon::new( data[1].trim().parse::<i32>().unwrap(),
                              data[3].trim().parse::<i32>().unwrap(),
                              data[5].trim().parse::<i32>().unwrap()));
    }
    moons
}

#[aoc(day12, part1)]
pub fn day12_pt1(data: &[Moon]) -> i32{
    //let mut moons = data.clone();
    let mut moons: Vec<Moon> = Vec::new();
    for m in data{
        moons.push(m.clone());
    }
    for _ in 0..1000{
        let mut data = Vec::new();
        for moon in moons.iter(){
            data.push(moon.values());
        }
        for moon in moons.iter_mut(){
            moon.gravity(&data);
            moon.step();
        }
    }
    let mut energy = 0;
    for moon in moons{
        let pot = moon.pot_energy();
        let kin = moon.kin_energy();
        energy += pot * kin;
    }
    energy
}
pub fn gcd(num1: usize, num2: usize) -> usize{
    if num2 == 0{
        return num1;
    }
    gcd(num2, num1 % num2)
}

pub fn lcm(data: &[usize]) -> usize{
    let mut ans = data[0];

    for val in data.iter().skip(1){
        ans = (ans * *val) / gcd(*val, ans);
    }
    ans
}

#[aoc(day12, part2)]
pub fn day12_pt2(data: &[Moon]) -> usize {
    let mut moons: Vec<Moon> = Vec::new();
    for m in data{
        moons.push(m.clone());
    }
    // get init data
    let mut init: Vec<(i32, i32, i32)> = vec![(0, 0, 0); moons.len()];
    for (i, m) in moons.iter().enumerate(){
        init[i] = m.values();
    }
    // find all cycles! 
    //let mut expected_cycles: Vec<usize> = vec![0; moons.len() * 3];
    let mut big_cycles = [0; 3];
    let mut found = 0;
    let mut curr_iter = 0;
    // Note that a cycle contains more than 1 iteration
    while found < big_cycles.len() {
        curr_iter += 1;
        let mut data = Vec::new();
        for moon in moons.iter() {
            data.push(moon.values());
        }
        let mut total_vel_x = 0;
        let mut total_vel_y = 0;
        let mut total_vel_z = 0;

        for (i, m) in moons.iter_mut().enumerate(){
            if curr_iter == 1 {
                println!("{}, {}, {}", m.x, m.y, m.z);
            }
            m.gravity(&data);
            m.step();
            let m_set = i * 3;
            total_vel_x += m.dx.abs();
            total_vel_y += m.dy.abs();
            total_vel_z += m.dz.abs();
        }
        if total_vel_x == 0 && big_cycles[0] == 0{
            big_cycles[0] = curr_iter * 2;
            found += 1;
        }
        if total_vel_y == 0 && big_cycles[1] == 0{
            big_cycles[1] = curr_iter * 2;
            found += 1;
        }
        if total_vel_z == 0 && big_cycles[2] == 0{
            big_cycles[2] = curr_iter * 2;
            found += 1;
        }
    } 
    // find LCM
    lcm(&big_cycles)
}
