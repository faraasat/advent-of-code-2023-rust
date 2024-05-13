use std::{collections::HashMap, fs, path::Path};

fn read_file_content(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("Unable to read file");
}

fn custom_sort<'a>(arr: &'a mut Vec<(&'a str, i32)>) -> Vec<(&'a str, i32)> {
    let priority_order = "AKQJT98765432";
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by_key(|s| {
        s.0.chars()
            .map(|c| priority_order.find(c).unwrap_or(priority_order.len()))
            .collect::<Vec<_>>()
    });

    sorted_arr.reverse();

    return sorted_arr.iter().map(|s| *s).collect::<Vec<_>>().clone();
}

fn main() {
    let content = read_file_content("src/input.txt");

    let content_list = content
        .lines()
        .into_iter()
        .map(|line| {
            let chunks = line.split(" ").collect::<Vec<_>>();
            return (chunks[0], chunks[1].parse::<i32>().unwrap());
        })
        .into_iter()
        .collect::<Vec<_>>()
        .clone();

    let mut five_of_a_kind = Vec::<(&str, i32)>::new();
    let mut four_of_a_kind = Vec::<(&str, i32)>::new();
    let mut full_house = Vec::<(&str, i32)>::new();
    let mut three_of_a_kind = Vec::<(&str, i32)>::new();
    let mut two_pair = Vec::<(&str, i32)>::new();
    let mut one_pair = Vec::<(&str, i32)>::new();
    let mut high_card = Vec::<(&str, i32)>::new();

    content_list.iter().for_each(|(a, b)| {
        let mut map = HashMap::<char, i32>::new();

        a.trim().chars().into_iter().for_each(|c| {
            let ch = c.clone();

            let in_map = map.get(&ch.clone()).unwrap_or(&0);

            map.insert(ch, in_map + 1);
        });

        if map.len() == 1 {
            five_of_a_kind.push((a, *b));
        } else if map.len() == 2 {
            let values = map.values().collect::<Vec<_>>();

            if values.contains(&&4) {
                four_of_a_kind.push((a, *b));
            } else {
                full_house.push((a, *b));
            }
        } else if map.len() == 3 {
            let values = map.values().collect::<Vec<_>>();

            if values.contains(&&3) {
                three_of_a_kind.push((a, *b));
            } else {
                two_pair.push((a, *b));
            }
        } else if map.len() == 4 {
            one_pair.push((a, *b));
        } else {
            high_card.push((a, *b));
        }
    });

    let sorted_content = vec![
        custom_sort(&mut high_card),
        custom_sort(&mut one_pair),
        custom_sort(&mut two_pair),
        custom_sort(&mut three_of_a_kind),
        custom_sort(&mut full_house),
        custom_sort(&mut four_of_a_kind),
        custom_sort(&mut five_of_a_kind),
    ]
    .concat();

    println!(
        "{:?}",
        sorted_content
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| {
                return acc + ((i.to_string().parse::<i32>().unwrap() + 1) * x.1);
            })
    );
}
