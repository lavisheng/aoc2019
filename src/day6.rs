use std::collections::HashMap;

// Use a hashmap to imitate a tree
// fix to make: minimize the uses of clone
pub fn input_gen(input: &str) -> HashMap<String, Vec<String>> {
    let mut orbital_data: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines(){
        let orbit = line.split(")").map(|x| String::from(x)).collect::<Vec<String>>();
        if !orbital_data.contains_key(&orbit[0]) {
            orbital_data.insert(orbit[0].clone(), Vec::new());
        }
        orbital_data.get_mut(&orbit[0])
            .unwrap()
            .push(orbit[1].clone());
        if !orbital_data.contains_key(&orbit[1]) {
            orbital_data.insert(orbit[1].clone(), Vec::new());
        }
    }
    orbital_data
}

// recursively traverse the hashmap
// assume node is in the hashmap
// depth: next depth down
pub fn traverse_depth(depth: usize,
                      node: String,
                      tree: &HashMap<String, Vec<String>>) -> usize{
    let mut out = 0;
    let children = tree.get(&node).unwrap();
    for child in children.iter(){
        out += traverse_depth(depth + 1, child.to_string(), tree);
    }
    out + children.len() * (depth + 1)
}

#[aoc(day6, part1)]
pub fn day6_pt1(input: &str) -> usize{
    // for some reason aoc wouldn't let me as ref a hashmap
    let tree = input_gen(input);
    //tree: &HashMap<String, Vec<String>>) -> usize {
    // we consider COM the root
    traverse_depth(0, String::from("COM"), &tree)
}

// returns -1 if not found, and positive if found
pub fn you_san_dist(node: String, tree: &HashMap<String, Vec<String>>)
                    -> (i32, String){
    if node == "YOU" || node == "SAN"{
        return (0, node);
    }
    let children = tree.get(&node).unwrap();
    let mut min_you = -1;
    let mut min_san = -1;
    let mut min_both = -1;
    for child in children.iter(){
        let (dist, id) = you_san_dist(child.to_string(), tree);
        // set the min variables appropriately
        if dist >= 0{
            match id.as_ref(){
                "YOUSAN" => {
                    if min_both == -1 || dist < min_both {
                        min_both = dist;
                    }
                },
                "YOU" => {
                    if min_you == -1 || dist < min_you{
                        min_you = dist;
                    }
                },
                "SAN" => {
                    if min_san == -1 || dist < min_san {
                        min_san = dist;
                    }
                },
                _ => println!("ERROR")
            }
        }
    }
    if min_both != -1 {
        return (min_both, "YOUSAN".to_string());
    } else if min_san != -1 && min_you != -1{
        return (min_san + min_you, "YOUSAN".to_string());
    } else if min_san != -1 {
        return (min_san + 1, "SAN".to_string());
    } else if min_you != -1{
        return (min_you + 1, "YOU".to_string());
    }
    // this value gets ignored
    (-1, "NONE".to_string())
}
#[aoc(day6, part2)]
pub fn day6_pt2(input: &str) -> usize{
    // again some weird issue with reference and hashmap
    let tree = input_gen(input);
    you_san_dist("COM".to_string(), &tree).0 as usize
}
