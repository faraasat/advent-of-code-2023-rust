use regex::Regex;
use std::{fs, path::Path};

fn read_file_content(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("Unable to read file");
}

fn get_series(line: &str) -> Vec<i32> {
    let re = Regex::new(r"[0-9]+").unwrap();

    return re.find_iter(line)
        .into_iter()
        .map(|x| x.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
}

fn main() {
    let content = read_file_content("src/input.txt");
    let content_lines = content.lines().into_iter().collect::<Vec<_>>();

    let time_list = get_series(content_lines[0]);
    let distance_list = get_series(content_lines[1]);
    let mut count_list = vec![0; 4];

    for i in 0..time_list.len() {
        let current_time = time_list[i];
        let current_distance = distance_list[i];
        let initial = true;

        for j in 1..current_time {
            let remaining_time = current_time - j;
            let distance_covered = remaining_time * j;

            if distance_covered >= current_distance {
                count_list[i] += 1;
            } else if !initial {
                break;
            }
        }
    }

    println!("{:?}", count_list.iter().product::<i32>());
}
