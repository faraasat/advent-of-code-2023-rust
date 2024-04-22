use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_file_input(file_path: &str) -> String {
    let content = fs::read_to_string(Path::new(file_path)).expect("Error reading file");

    return content;
}

fn main() {
    let file_content = read_file_input("./src/input.txt");
    let num_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let num_list = num_map.clone().into_keys().collect::<Vec<&str>>();
    let my_len = file_content.lines().count();
    let mut arr1: Vec<i32> = vec![0; my_len];
    let mut count = 0;
    let mut sum = 0;

    for lines in file_content.lines() {
        let mut digit_list = vec![];
        let mut digits: String = String::from("");

        lines.chars().for_each(|x| {
            if x.is_numeric() {
                digit_list.push(x as i32 - 0x30);
                digits.clear();
            } else {
                digits.push(x);
                if !num_list.iter().any(|&item| item.starts_with(&digits)) {
                    digits.remove(0);
                } else if num_list.contains(&digits.as_str()) {
                    digit_list.push(*num_map.get(&digits as &str).unwrap() as i32);
                    let new_digit = digits.pop().unwrap();
                    digits.clear();
                    digits.push(new_digit);
                }
            }
        });
        println!("{:?}", digit_list);

        arr1[count] = (digit_list[0] * 10) + digit_list[digit_list.len() - 1];
        count = count + 1;
    }

    // println!("{:?}", arr1);

    for i in arr1.iter() {
        sum = sum + i;
    }

    println!("{:?}", sum);
}
