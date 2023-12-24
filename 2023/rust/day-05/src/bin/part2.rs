fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let first_part = input
        .split("\n\nseed-to-soil map:\n")
        .collect::<Vec<&str>>();
    let seed_elems = first_part[0][7..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut seed_pairs = vec![];
    for i in 1..seed_elems.len() {
        if i % 2 == 1 {
            seed_pairs.push((seed_elems[i - 1], seed_elems[i - 1] + seed_elems[i]));
        }
    }

    let second_part = first_part[1]
        .split("\n\nsoil-to-fertilizer map:\n")
        .collect::<Vec<&str>>();
    let mut seed_to_soil = vec![];
    for line in second_part[0].lines() {
        seed_to_soil.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let third_part = second_part[1]
        .split("\n\nfertilizer-to-water map:\n")
        .collect::<Vec<&str>>();
    let mut soil_to_fertilizer = vec![];
    for line in third_part[0].lines() {
        soil_to_fertilizer.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let fourth_part = third_part[1]
        .split("\n\nwater-to-light map:\n")
        .collect::<Vec<&str>>();
    let mut fertilizer_to_water = vec![];
    for line in fourth_part[0].lines() {
        fertilizer_to_water.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let fifth_part = fourth_part[1]
        .split("\n\nlight-to-temperature map:\n")
        .collect::<Vec<&str>>();
    let mut water_to_light = vec![];
    for line in fifth_part[0].lines() {
        water_to_light.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let sixth_part = fifth_part[1]
        .split("\n\ntemperature-to-humidity map:\n")
        .collect::<Vec<&str>>();
    let mut light_to_temperature = vec![];
    for line in sixth_part[0].lines() {
        light_to_temperature.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let seventh_part = sixth_part[1]
        .split("\n\nhumidity-to-location map:\n")
        .collect::<Vec<&str>>();
    let mut temperature_to_humidity = vec![];
    for line in seventh_part[0].lines() {
        temperature_to_humidity.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let mut humidity_to_location = vec![];
    for line in seventh_part[1].lines() {
        humidity_to_location.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let map_function = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    // let mut location_nums = vec![];
    // for (start, end) in seed_pairs {
    //     for seed in start..end {
    //         location_nums.push(search_for_mapping(seed, &map_function, 0));
    //     }
    //     print!("another one done");
    // }
    // let mut smallest_num = *location_nums.first().unwrap();
    // for num in location_nums {
    //     if num < smallest_num {
    //         smallest_num = num;
    //     }
    // }

    for i in 1..std::u64::MAX {
        let seed_number = search_for_mapping_backwards(i, &map_function, 7);
        println!("i: {}, seed_number: {}", i, seed_number);
        for (start, end) in &seed_pairs {
            if *start <= seed_number && seed_number < *end {
                println!("start: {}, end: {}", start, end);
                return i;
            }
        }
    }
    return 0;
}
/*
fn search_for_mapping(element: u64, map_function: &[Vec<Vec<u64>>; 7], counter: usize) -> u64 {
    let mut mapping = element;
    for partial_map in &map_function[counter] {
        let domain_num = partial_map[1];
        if domain_num <= element && element < domain_num + partial_map[2] {
            mapping = partial_map[0] + element - domain_num;
        }
    }
    if counter < 6 {
        search_for_mapping(mapping, map_function, counter + 1)
    } else {
        mapping
    }
}
*/
fn search_for_mapping_backwards(
    element: u64,
    map_function: &[Vec<Vec<u64>>; 7],
    counter: usize,
) -> u64 {
    let mut mapping = element;
    for partial_map in &map_function[counter - 1] {
        let domain_num = partial_map[0];
        if domain_num <= element && element < domain_num + partial_map[2] {
            mapping = partial_map[1] + element - domain_num;
        }
    }
    if counter > 1 {
        search_for_mapping_backwards(mapping, map_function, counter - 1)
    } else {
        mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part2(test_case);
        assert_eq!(result, 46u64);
    }
}
