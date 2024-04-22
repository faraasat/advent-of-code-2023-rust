use std::collections::HashMap;
use std::path::Path;
use std::fs;

fn read_file_input(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn main() {
    let content = read_file_input("./src/input.txt");
    let symbol_list = ["!", "@", "#", "$", "%", "^", "&", "*", "-", "+", "/", "%"];
    let mut symbol_pos: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut line_count = 0;

    for line in content.lines() {
        let mut internal_count = 0;
        for i in line.chars() {
            internal_count += 1;

            if symbol_list.contains(&i.to_string().as_str()) {
                let mut num_list = Vec::new();
                if !symbol_pos.get(&internal_count).is_none() {
                    if symbol_pos.get(&internal_count).clone().unwrap().len() > 0 {
                        num_list = symbol_pos.get(&line_count).unwrap().clone();
                    }
                }
                num_list.push(internal_count);
                symbol_pos.insert(line_count, num_list);
            }
        }

        line_count += 1;
    }

    println!("{:?}", symbol_pos);
}
