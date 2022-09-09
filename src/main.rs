use std::{env, fs};

fn main() {
    let path = get_path_from_args(env::args().collect()).expect("need path as argument");

    let content = read_from_file(path.as_str());
    let (lines, width) = parse_lines(content);
    let gamma = get_gamma_rate(lines, width);
    let epsilon = get_epsilon_rate(gamma, width);
    println!("{:} {:} {:}", gamma, epsilon, gamma * epsilon);
}

fn get_path_from_args(args: Vec<String>) -> Option<String> {
    match args.len() {
        2 => Some(args[1].to_owned()),
        _ => None,
    }
}

fn read_from_file(path: &str) -> String {
    fs::read_to_string(path).expect("read file error")
}

fn parse_lines(content: String) -> (Vec<u32>, u32) {
    let iter = content.split('\n');
    let width = iter.clone().next().expect("no data").len();
    (
        iter.map(|x| u32::from_str_radix(x, 2).expect("NaN"))
            .collect(),
        width.try_into().unwrap(),
    )
}

fn get_gamma_rate(list: Vec<u32>, width: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..width {
        let mut counter = 0;
        for line in &list {
            if (line & (1 << i)) == (1 << i) {
                counter += 1;
            }
        }

        if counter > (list.len() / 2) {
            result |= 1 << i;
        }
    }

    result
}

fn get_epsilon_rate(gamma: u32, width: u32) -> u32 {
    let mut result = gamma;
    for i in 0..width {
        result ^= 1 << i;
    }
    result
}
