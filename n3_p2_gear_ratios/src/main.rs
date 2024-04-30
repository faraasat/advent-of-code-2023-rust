use std::fs;
use std::path::Path;
use regex::Regex;

fn usize_to_i32(num: usize) -> i32 {
    return num.to_string().parse::<i32>().unwrap();
}

fn str_to_i32(num: &str) -> i32 {
    return num.parse::<i32>().unwrap();
}

fn read_file_input(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn extract_number_position_range(content: &str, i: usize, num_list: &mut Vec<(Vec<i32>, i32)>) {
    let re2 = Regex::new(r"[0-9]+").unwrap();

    re2.find_iter(&content.lines().into_iter().nth(i).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| (x.start(), x.end(), str_to_i32(x.as_str())))
        .for_each(|(start, end, val)| {
            num_list.push((
                ((usize_to_i32(start))..(usize_to_i32(end) + 2))
                    .into_iter()
                    .collect::<Vec<i32>>(),
                val,
            ));
            // print!(
            //     "{} -> {:?} -> {:?} \t",
            //     val,
            //     ((usize_to_i32(start))..(usize_to_i32(end) + 2))
            //         .into_iter()
            //         .collect::<Vec<i32>>(),
            //     (start, end, val)
            // );
        });
    // println!("");
}

fn main() {
    let content = read_file_input("./src/input.txt");
    let re = Regex::new(r"[0-9]+[*][0-9]+").unwrap();
    let re3 = Regex::new(r"[0-9]{0}[*][0-9]{0}").unwrap();
    let mut final_list: Vec<i32> = vec![];
    let mut final_list_test: Vec<(i32, i32)> = vec![];
    let matches = re.find_iter(&content).collect::<Vec<_>>();

    for m in matches {
        let sample = m.as_str().split("*").collect::<Vec<&str>>();

        final_list.push(str_to_i32(sample[0]) * str_to_i32(sample[1]));
    }

    for i in 0..content.lines().count() - 1 {
        if i == 0 {
            continue;
        }
        let mut num_list_1: Vec<(Vec<i32>, i32)> = vec![];
        let mut num_list_2: Vec<(Vec<i32>, i32)> = vec![];

        let symbol_list = re3
            .find_iter(&content.lines().into_iter().nth(i).unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| return x.end())
            .collect::<Vec<_>>();

        extract_number_position_range(&content, i - 1, &mut num_list_1);
        extract_number_position_range(&content, i + 1, &mut num_list_2);

        // println!("{:?}", symbol_list);

        // break;

        for s in symbol_list.clone() {
            let mut found_1: &(Vec<i32>, i32) = &(Vec::new(), 0);
            let mut found_2: &(Vec<i32>, i32) = &(Vec::new(), 0);

            for i in &num_list_1 {
                if i.0.contains(&usize_to_i32(s)) {
                    found_1 = i;
                    break;
                }
            }

            for i in &num_list_2 {
                if i.0.contains(&usize_to_i32(s)) {
                    found_2 = i;
                    break;
                }
            }

            if found_1.1 > 0 && found_2.1 > 0 {
                final_list.push(found_1.1 * found_2.1);
                // print!("{:?} * {:?}\t", found_1.1, found_2.1);
                final_list_test.push((found_1.1, found_2.1));
                // print!("{} * {}\t", found_1.1, found_2.1);
            }
        }
        println!("");

        // println!("num_list_1: {:?} +++++++++ ", num_list_1);
        // println!("symbol_list: {:?} +++++++++ ", symbol_list);
        // println!("num_list_2: {:?} +++++++++ ", num_list_2);
        // println!(
        //     "==================================================================================\n"
        // );
    }

    // println!("{:?}", final_list_test);
    // println!("{:?}", final_list);
    println!("\n{:?}", final_list.iter().sum::<i32>());
}
