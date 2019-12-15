use std::fs::read_to_string;
use std::u32;

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;

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

fn flatten_layers(layers: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut flat = vec![TRANSPARENT; layers.first().unwrap().len()];
    for layer in layers.iter() {
        for (i, px) in layer.iter().enumerate() {
            if flat[i] == TRANSPARENT && *px != TRANSPARENT {
                flat[i] = *px;
            }
        }
    }

    return flat;
}

fn render_layer(width: usize, height: usize, data: &Vec<u32>) {
    // render the final image
    for i in 0..height {
        for j in 0..width {
            match data[width * i + j] {
                BLACK => print!("  "),
                WHITE => print!("██"),
                TRANSPARENT | _ => print!("  "),
            }
        }
        println!("");
    }
}

fn parse_input(file_path: &str) -> Vec<u32> {
    return read_to_string(file_path)
    .expect("unable to read file")
    .trim()
    .chars()
    .map(|c| c.to_digit(10).expect("failed to parse char {}"))
    .collect();
}

fn split_layers(width: usize, height: usize, data: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut layers: Vec<Vec<u32>> = Vec::new();
    // iterate over every layer
    for li in 0..layer_count(width, height, &data) {
        let mut layer: Vec<u32> = Vec::new();
        // iterate over every pixel in the layer
        for i in 0..(width * height) {
            // calculate the actual pixel index in data vec
            let pi = (width * height * li) + i;
            layer.push(data[pi]);
        }
        layers.push(layer);
    }
    return layers;
}

fn part1(layers: &Vec<Vec<u32>>) -> u32 {
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
    return count_1 * count_2;
}

fn main() {
    let width = 25;
    let height = 6;
    // read input
    let data = parse_input("input.txt");
    // split image into layers
    let layers = split_layers(width, height, &data);

    // part 1
    let p1_ans = part1(&layers);
    println!("Part1 Answer: {}", p1_ans);

    // part 2
    let final_img = flatten_layers(&layers);
    render_layer(width, height, &final_img);
}
