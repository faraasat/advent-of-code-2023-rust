use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_file_text(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn main() {
    let content = read_file_text("./src/input.txt");
    let mut cube_pow :Vec<i32>= vec![1; content.lines().count()];
    let mut count = 0;

    for i in content.lines() {
        let split_str = i.split(":").collect::<Vec<&str>>();
        let row = split_str[1].trim();
        let mut max_cube_det = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for i in row.split("; ").collect::<Vec<&str>>() {
            for j in i.split(", ").collect::<Vec<&str>>() {
                let sp = j.split(" ").collect::<Vec<&str>>();
                // game_number.insert(sp[1], sp[0]);
                if sp[0].parse::<i32>().unwrap() > *max_cube_det.get(sp[1]).unwrap() as i32 {
                    max_cube_det.insert(sp[1], sp[0].parse::<i32>().unwrap());
                }
            }
        }

        for res in max_cube_det.values().cloned().collect::<Vec<i32>>() {
            cube_pow[count] *= res;
        }
        count += 1;
    }

    let total = cube_pow.iter().sum::<i32>();

    println!("{:?}", total);
}
