use std::{collections::HashMap, fs, path::Path};

fn read_content_string(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("Unable to read file");
}

fn main() {
    let content = read_content_string("src/input.txt");

    let content_vec = content
        .lines()
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut direction = content_vec[0].chars().collect::<Vec<_>>();
    let mut map_dir = HashMap::<&str, (&str, &str)>::new();

    content_vec
        .clone()
        .drain(1..)
        .collect::<Vec<_>>()
        .iter()
        .for_each(|x| {
            let chunks = x.split(" = (").collect::<Vec<_>>();
            let key = chunks[0];
            let value = chunks[1].split(")").collect::<Vec<_>>()[0]
                .split(", ")
                .collect::<Vec<_>>();
            map_dir.insert(key, (value[0], value[1]));
        });

    direction.extend(direction.clone());

    let mut current = map_dir.get("AAA").unwrap();
    let mut count = 0;
    let mut iter = 0;

    loop {
        let mut next: &str = "";

        if direction[iter] == 'L' {
            next = current.0;
        } else if direction[iter] == 'R' {
            next = current.1;
        }

        current = map_dir.get(next).unwrap();
        count += 1;
        iter += 1;
        if iter == direction.len() {
            iter = 0;
        }

        if next == "ZZZ" {
            break;
        }
    }

    println!("{:?}", count);
}
