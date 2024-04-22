use std::fs;
use std::path::Path;

fn read_file_input() -> String {
    let contents = fs::read_to_string(Path::new("./src/input.txt")).expect("Error reading file");

    return contents;
}

fn main() {
    let input = read_file_input();
    let my_len = input.lines().count();
    let mut arr1: Vec<i32> = vec![0; my_len];
    let mut count = 0;
    let mut sum = 0;

    for i in input.lines() {
        let mut is_first = true;
        let mut first_num: i32 = -1;
        let mut last_num = -1;

        i.chars().for_each(|x| {
            if x.is_numeric() {
                if is_first {
                    first_num = x as i32 - 0x30;
                    arr1[count] = first_num * 10;
                    is_first = false;
                } else {
                    last_num = x as i32 - 0x30;
                    return;
                }
            }
        });
        if last_num == -1 {
            arr1[count] = arr1[count] + first_num;
        } else {
            arr1[count] = arr1[count] + last_num;
        }
        count = count + 1;
    }

    for i in arr1.iter() {
        sum = sum + i;
    }

    println!("{:?}", arr1);
    println!("{:#?}", sum);
}
