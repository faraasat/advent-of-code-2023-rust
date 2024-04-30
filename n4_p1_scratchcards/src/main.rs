use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn read_file_content(path: &str) -> String {
    let content = fs::read_to_string(Path::new(path)).expect("Error Reading File");

    return content;
}

fn main() {
    let content = read_file_content("./src/input.txt");
    let re = Regex::new(r"\s\s\s|\s\s|\s").unwrap();
    let mut total = 0;

    for line in content.lines() {
        let card = (line.split(": ").collect::<Vec<_>>()[1])
            .split(" | ")
            .collect::<Vec<_>>();
        let winning_numbers = re.split(card[0]).filter(|x| !x.is_empty()).collect::<Vec<_>>();
        let your_numbers = re.split(card[1]).filter(|x| !x.is_empty()).collect::<Vec<_>>();
        let intersection: HashSet<_> = winning_numbers
            .clone()
            .into_iter()
            .filter(|x| your_numbers.contains(x))
            .collect::<HashSet<_>>();

        if intersection.len() > 0 {
            let win_count = i32::pow(
                2,
                intersection.len().to_string().parse::<u32>().unwrap() - 1,
            );
            total = total + win_count;
        }
    }

    println!("{:?}", total);
}
