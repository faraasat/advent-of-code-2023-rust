use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_file_input(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn main() {
    let content = read_file_input("./src/input.txt");
    let symbol_list = ["!", "@", "#", "$", "%", "^", "&", "*", "-", "+", "/", "%", "="];
    let mut symbol_pos: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut num_pos: HashMap<i32, Vec<HashMap<i32, String>>> = HashMap::new();
    let mut line_count = 0;

    for line in content.lines() {
        let mut internal_count = 0;
        let mut num_list = Vec::new();
        let mut num_pos_list = Vec::<HashMap<i32, String>>::new();
        let mut is_num_found = false;

        for i in line.chars() {
            if symbol_list.contains(&i.to_string().as_str()) {
                num_list.push(internal_count);
                symbol_pos.insert(line_count, num_list.clone());
                is_num_found = false;
            }

            if !symbol_list.contains(&i.to_string().as_str()) && !i.is_numeric() {
                is_num_found = false;
            }

            if i.is_numeric() && !is_num_found {
                let mut ic = internal_count + 1;
                let mut num = Vec::from([i]);

                loop {
                    let ch = line
                        .chars()
                        .nth(ic as usize)
                        .unwrap_or(" ".chars().next().unwrap());
                    if ch.is_numeric() {
                        num.push(ch);
                        is_num_found = true;
                    } else {
                        break;
                    }
                    ic += 1;
                }

                let mut num_pos_map = HashMap::new();
                num_pos_map.insert(
                    num.iter().collect::<String>().parse::<i32>().unwrap(),
                    internal_count.to_string()
                        + ","
                        + (ic - internal_count - 1).to_string().as_str(),
                );
                num_pos_list.push(num_pos_map);
            }

            num_pos.insert(line_count, num_pos_list.clone());
            internal_count += 1;
        }

        line_count += 1;
    }

    // println!("{:?}", symbol_pos);
    // println!("{:?}", num_pos);

    let mut final_num_list = Vec::new();

    for i in 0..content.lines().count() {
        let iteration = i.to_string().parse::<i32>().unwrap();
        let num_positions = num_pos.get(&iteration).unwrap_or(&Vec::new()).clone();

        for num_list in num_positions {
            for (num, pos_chars) in num_list {
                let pos_list = pos_chars.split(",").collect::<Vec<&str>>();
                let pos = pos_list[0].parse::<i32>().unwrap();
                let pad = pos_list[1].parse::<i32>().unwrap();

                for j in -1..pad + 2 {
                    if symbol_pos
                        .get(&iteration)
                        .unwrap_or(&Vec::new())
                        .clone()
                        .contains(&(pos + j))
                    {
                        final_num_list.push(num);
                    }
                }

                if i != 0 {
                    for j in -1..pad + 2 {
                        if symbol_pos
                            .get(&(iteration - 1))
                            .unwrap_or(&Vec::new())
                            .clone()
                            .contains(&(pos + j))
                        {
                            final_num_list.push(num);
                        }
                    }
                }

                if i != content.lines().count() - 1 {
                    for j in -1..pad + 2 {
                        if symbol_pos
                            .get(&(iteration + 1))
                            .unwrap_or(&Vec::new())
                            .clone()
                            .contains(&(pos + j))
                        {
                            final_num_list.push(num);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", final_num_list.iter().sum::<i32>());
}
