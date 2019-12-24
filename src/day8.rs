use colored::*;
#[aoc(day8, part1)]
pub fn day8_pt1(input: &str) -> usize{
    let pixels = input.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    // number of pixels per layer, ignoring width for now
    let per_layer = 6 * 25;
    //let mut memory = [u32; per_layer];
    let mut min: i32 = -1;
    let mut product = 0;


    for layer in 0..pixels.len() / per_layer{
        let pixel_layer = &pixels[(layer * per_layer)..(layer + 1) * per_layer];
        let zeroes = pixel_layer.iter()
            .filter(|z| **z == 0)
            .count();
        if min == -1 || (zeroes as i32)< min {
            min = zeroes as i32;
            let (ones, twos): (Vec<u32>, Vec<u32>) = pixel_layer.iter()
                .filter(|v| **v == 1 || **v == 2)
                .partition(|&&v| v == 1);
            product = ones.len() * twos.len();
        }
    }
    product
}

// prints the image! 
pub fn print_img(image: &[ [u32; 25]; 6]){
    for row in image{
        for pixel in row{
            if *pixel != 2{
                if *pixel == 1{
                    print!("{}", pixel.to_string().red().bold());
                } else if *pixel == 0{
                    print!("{}", pixel.to_string().yellow());
                }
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
    println!(" ");
    println!(" ");
}
#[aoc(day8, part2)]
pub fn day8_pt2(input: &str) -> String{
    let pixels = input.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut image: [[u32; 25]; 6] = [[2; 25]; 6];
    let per_layer = 6 * 25;
    for layer in 0..pixels.len() / per_layer{
        let pixel_layer = &pixels[(layer * per_layer)..(layer + 1) * per_layer];
        for row in 0..6{
            for column in 0..25{
                if image[row][column] == 2{
                    image[row][column] = pixel_layer[row * 25 + column];
                }
            }
        }
    }
    print_img(&image);
    "done!".to_string()
}
