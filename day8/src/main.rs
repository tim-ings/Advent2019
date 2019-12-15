use std::fs::read_to_string;
use std::u32;

fn layer_count(width: usize, height: usize, data: &Vec<u32>) -> usize {
    data.len() / (width * height)
}

fn count_digit_occurence(digit: u32, data: &Vec<u32>) -> u32 {
    let mut count = 0;
    for x in data.iter() {
        if *x == digit {
            count += 1;
        }
    }
    return count;
}

#[allow(dead_code)]
fn print_layer(width: usize, height: usize, data: &Vec<u32>) {
    for i in 0..height {
        for j in 0..width {
            print!("{}", data[height * i + j]);
        }
        println!("");
    }
}

fn main() {
    let width = 25;
    let height = 6;
    // read input
    let data: Vec<u32> = read_to_string("input.txt")
              .expect("unable to read file")
              .trim()
              .chars()
              .map(|c| c.to_digit(10).expect("failed to parse char {}"))
              .collect();
    // split image into layers
    let mut layers: Vec<Vec<u32>> = Vec::new();
    // iterate over every layer
    for li in 0..layer_count(width, height, &data) {
        let mut layer: Vec<u32> = Vec::new();
        // iterate over every pixel in the layer
        for i in 0..width * height {
            // calculate the actual pixel index in data
            let pi = (width * height * li) + i;
            layer.push(data[pi]);
        }
        // println!("Layer {}", li);
        // print_layer(width, height, &layer);
        layers.push(layer);
    }

    // find layer with fewest 0's
    let mut fewest_index = 0;
    let mut fewest_count = u32::MAX;
    for (i, layer) in layers.iter().enumerate() {
        let new_count = count_digit_occurence(0, &layer);
        if new_count < fewest_count {
            fewest_count = new_count;
            fewest_index = i;
        }
    }

    // get number of 1's and 2's
    let count_1 = count_digit_occurence(1, &layers[fewest_index]);
    let count_2 = count_digit_occurence(2, &layers[fewest_index]);
    println!("Part1 Answer: {}", count_1 * count_2);
}
