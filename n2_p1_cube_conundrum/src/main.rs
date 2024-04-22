use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_file_text(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn main() {
    let content = read_file_text("./src/input.txt");
    let mut possible_ids_sum = 0;
    let cube_det = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for i in content.lines() {
        let split_str = i.split(":").collect::<Vec<&str>>();
        let row = split_str[1].trim();
        let id = split_str[0].split(" ").collect::<Vec<&str>>()[1]
            .split(":")
            .collect::<Vec<&str>>()[0]
            .trim();
        // let mut game_number = HashMap::<&str, &str>::new();
        let mut is_possible = true;

        for i in row.split("; ").collect::<Vec<&str>>() {
            for j in i.split(", ").collect::<Vec<&str>>() {
                let sp = j.split(" ").collect::<Vec<&str>>();
                // game_number.insert(sp[1], sp[0]);
                if sp[0].parse::<i32>().unwrap() > *cube_det.get(sp[1]).unwrap() as i32 {
                    is_possible = false;
                    break;
                }
            }
        }

        // for i in game_number.into_iter().collect::<Vec<(&str, &str)>>() {
        //     if i.1.parse::<i32>().unwrap() > *cube_det.get(i.0).unwrap() as i32 {
        //         is_possible = false;
        //         break;
        //     }
        // }

        if is_possible {
            possible_ids_sum += id.parse::<i32>().unwrap();
        }
    }

    println!("{:?}", possible_ids_sum)
}
