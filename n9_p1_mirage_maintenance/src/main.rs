use std::{fs, path::Path};

fn read_file_content(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("Unable to read file");
}

fn main() {
    let content = read_file_content("src/input.txt");

    let content_vec = content
        .lines()
        .into_iter()
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = Vec::<i64>::new();

    for it in 0..content_vec.len() {
        let a = &content_vec.clone()[it];
        let mut new_vec = Vec::<Vec<i64>>::new();
        new_vec.push(a.to_vec());
        let mut count = 0;

        loop {
            let current_vec = &new_vec.clone()[count];
            new_vec.push(Vec::new());
            let mut zero_flag = false;

            for i in 0..current_vec.len() - 1 {
                let new_calc = current_vec[i + 1] - current_vec[i];
                if new_calc == 0 {
                    zero_flag = true;
                } else {
                    zero_flag = false;
                }
                new_vec[count + 1].push(current_vec[i + 1] - current_vec[i]);
            }

            count += 1;

            if zero_flag {
                break;
            }
        }

        loop {
            let current = new_vec[count].clone();
            let prev = new_vec[count - 1].clone();

            new_vec[count - 1].push(prev[prev.len() - 1] + current[current.len() - 1]);

            count -= 1;

            if count == 0 {
                break;
            }
        }

        let first = new_vec[0].clone();
        result.push(first[first.len() - 1]);
    }

    println!("{:?}", result.iter().sum::<i64>());
}
