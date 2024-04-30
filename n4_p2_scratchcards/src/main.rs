use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn read_file_content(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error reading file");

    return content;
}

fn main() {
    let content = read_file_content("./src/input.txt");
    let re = Regex::new(r"\s\s\s|\s\s|\s").unwrap();
    let mut matching_card_count = Vec::<i32>::new();
    let mut card_match = HashMap::<i32, i32>::new();

    for line in content.lines() {
        let card = (line.split(": ").collect::<Vec<_>>()[1])
            .split(" | ")
            .collect::<Vec<_>>();
        let winning_numbers = re
            .split(card[0])
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let your_numbers = re
            .split(card[1])
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let intersection: HashSet<_> = winning_numbers
            .clone()
            .into_iter()
            .filter(|x| your_numbers.contains(x))
            .collect::<HashSet<_>>();

        matching_card_count.push(intersection.len() as i32);
    }

    for i in 0..matching_card_count.len() {
        let num_i = card_match.get(&(i as i32)).unwrap_or_else(|| &0).clone();

        if num_i == 0 {
            card_match.insert(i as i32, 1);
        } else {
            card_match.insert(i as i32, num_i + 1);
        }

        for j in (i + 1)..(i + (matching_card_count[i] + 1) as usize) {
            let initial_count = card_match.get(&(j as i32));
            if initial_count.is_none() {
                if num_i == 0 {
                    card_match.insert(j as i32, 1);
                } else {
                    card_match.insert(j as i32, 1 + (1 * num_i));
                }
            } else {
                if num_i == 0 {
                    card_match.insert(j as i32, initial_count.unwrap() + 1);
                } else {
                    card_match.insert(j as i32, initial_count.unwrap() + 1 + (1 * num_i));
                }
            }
        }
    }

    println!("{:?}", card_match.values().sum::<i32>());
}
