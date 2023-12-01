use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use fancy_regex::Regex;

pub fn part1(reader: BufReader<File>) -> String {
    let mut sum = 0;

    for line in reader.lines() {
        let line_text = line.unwrap();
        // print!("{}\n", line_text);
        let mut _first_number: char = 'x';
        let mut first_number_set = false;
        let mut _second_number: char = 'y';

        for character in line_text.chars() {
            if character.is_numeric() {
                if !first_number_set {
                    _first_number = character;
                    _second_number = character;
                    first_number_set = true;
                } else {
                    _second_number = character
                }
            }
        }

        let combined_string = _first_number.to_string() + &_second_number.to_string();
        // print!("{}\n", combined_string);
        let computed_value: u32 = combined_string.parse().unwrap();

        // print!("{}", computed_value);
        sum = sum + computed_value;
    }

    sum.to_string()
}

// pub fn part2(reader: BufReader<File>) -> String {
//     let mut sum = 0;

//     let words_to_search = [
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
//         "5", "6", "7", "8", "9",
//     ];

//     let pattern_str = words_to_search.join("|");
//     let pattern = Regex::new(&pattern_str).unwrap();

//     for line in reader.lines() {
//         let line_text = line.unwrap();
//         let line_text = line_text
//             .to_lowercase()
//             .replace("one", "o1e")
//             .replace("two", "t2o")
//             .replace("three", "t3e")
//             .replace("four", "f4r")
//             .replace("five", "f5e")
//             .replace("six", "s6x")
//             .replace("seven", "s7n")
//             .replace("eight", "e8t")
//             .replace("nine", "n9e");

//         let mut _first_number: String = String::from("N/A");
//         let mut first_number_set = false;
//         let mut _second_number: String = String::from("N/A");

//         // println!("{}", line_text);
//         for mat in pattern.find_iter(&line_text) {
//             if !first_number_set {
//                 let matched = mat.unwrap().as_str();
//                 _first_number = matched.to_owned();
//                 first_number_set = true;
//                 _second_number = matched.to_owned();
//             } else {
//                 _second_number = mat.unwrap().as_str().to_owned();
//             }
//         }

//         // println!(
//         //     "first number: {}, second number: {}",
//         //     _first_number, _second_number
//         // );

//         let parsed_first = parse_number(_first_number);
//         let parsed_second = parse_number(_second_number);

//         // println!(
//         //     "first parsed: {}, second parsed: {}",
//         //     parsed_first, parsed_second
//         // );

//         let combined_string = parsed_first.to_string() + &parsed_second.to_string();
//         // println!("combined_string {}", combined_string);
//         let computed_value: u32 = combined_string.parse().unwrap();
//         // println!("computed_value {}", computed_value);

//         sum = sum + computed_value;
//     }

//     sum.to_string()
// }

pub fn part2(reader: BufReader<File>) -> String {
    let mut sum = 0;

    for line in reader.lines() {
        let line_text = line
            .unwrap()
            .to_lowercase()
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        // print!("{}\n", line_text);
        let mut _first_number: char = 'x';
        let mut first_number_set = false;
        let mut _second_number: char = 'y';

        for character in line_text.chars() {
            if character.is_numeric() {
                if !first_number_set {
                    _first_number = character;
                    _second_number = character;
                    first_number_set = true;
                } else {
                    _second_number = character
                }
            }
        }

        let combined_string = _first_number.to_string() + &_second_number.to_string();
        // print!("{}\n", combined_string);
        let computed_value: u32 = combined_string.parse().unwrap();

        // print!("{}", computed_value);
        sum = sum + computed_value;
    }

    sum.to_string()
}

fn parse_number(text: String) -> i32 {
    let mut value_hashmap: HashMap<String, i32> = HashMap::new();
    value_hashmap.insert("one".to_string(), 1);
    value_hashmap.insert("two".to_string(), 2);
    value_hashmap.insert("three".to_string(), 3);
    value_hashmap.insert("four".to_string(), 4);
    value_hashmap.insert("five".to_string(), 5);
    value_hashmap.insert("six".to_string(), 6);
    value_hashmap.insert("seven".to_string(), 7);
    value_hashmap.insert("eight".to_string(), 8);
    value_hashmap.insert("nine".to_string(), 9);

    if text.parse::<i32>().is_ok() {
        return text.parse::<i32>().unwrap();
    } else {
        return value_hashmap[&text];
    }
}
