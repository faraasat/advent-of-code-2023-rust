use std::fs;
use std::path::Path;
use std::thread;

fn read_file_content(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("Unable to read file");
}

fn str_to_u64(s: &str) -> u64 {
    return s.to_string().parse::<u64>().unwrap();
}

fn serialize_to_range_tuples(elem: &str) -> Vec<(u64, u64, u64)> {
    return elem.split(":\r\n").collect::<Vec<_>>()[1]
        .split("\r\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| {
            let sl = x.split(" ").collect::<Vec<&str>>();
            let src = str_to_u64(sl[0]);
            let dest = str_to_u64(sl[1]);
            let range = str_to_u64(sl[2]);

            return (src, dest, range);
        })
        .collect::<Vec<_>>();
}

fn find_mapping(parent: Vec<u64>, tuple: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    let mut map = Vec::<u64>::new();

    for i in parent {
        let mut flag: bool = false;

        for j in tuple.clone() {
            if (i >= j.1) && (i <= (j.1 + j.2)) {
                let dest = j.0 + (i - j.1);
                map.push(dest);
                flag = true;
                break;
            }
        }

        if !flag {
            map.push(i);
        }
    }

    return map;
}

fn main() {
    let content = read_file_content("src/input.txt");

    let blocks = content.split("\r\n\r\n").collect::<Vec<_>>();

    let seeds = blocks[0].split(": ").collect::<Vec<_>>()[1]
        .split(" ")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| str_to_u64(x))
        .collect::<Vec<u64>>();

    let handle_threads: Vec<_> = (1..8)
        .map(|i| {
            let content = content.clone();
            thread::spawn(move || {
                let blocks = content.split("\r\n\r\n").collect::<Vec<_>>();
                let block = blocks[i];
                serialize_to_range_tuples(&block)
            })
        })
        .collect();

    let handle = handle_threads
        .into_iter()
        .enumerate()
        .map(|x| x.1.join().unwrap())
        .collect::<Vec<_>>();

    let seed_to_soil_map = find_mapping(seeds, &handle[0]);
    let soil_to_fertilizer_map = find_mapping(seed_to_soil_map, &handle[1]);
    let fertilizer_to_water_map = find_mapping(soil_to_fertilizer_map, &handle[2]);
    let water_to_light_map = find_mapping(fertilizer_to_water_map, &handle[3]);
    let light_to_temperature_map = find_mapping(water_to_light_map, &handle[4]);
    let temperature_to_humidity_map = find_mapping(light_to_temperature_map, &handle[5]);
    let humidity_to_location_map = find_mapping(temperature_to_humidity_map, &handle[6]);

    println!("{:?}", humidity_to_location_map.iter().min().unwrap_or(&0));
}
