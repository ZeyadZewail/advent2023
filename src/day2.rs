use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(reader: BufReader<File>) -> String {
    let mut colors_maxcount: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs into the hashmap
    colors_maxcount.insert("red".to_string(), 12);
    colors_maxcount.insert("green".to_string(), 13);
    colors_maxcount.insert("blue".to_string(), 14);

    let mut sum = 0;

    for line in reader.lines() {
        let line_text = line.unwrap();

        let segments: Vec<_> = line_text.split(':').collect();
        let game_id: i32 = segments[0].split(' ').nth(1).unwrap().parse().unwrap();

        let round_segments: Vec<Vec<_>> = segments[1]
            .trim()
            .split(";")
            .map(|x| x.split(", ").map(|x| x.trim()).collect())
            .collect();
        let mut turn_viable = true;

        for turn in round_segments.clone() {
            for pull in turn {
                let split_pull: Vec<_> = pull.split(' ').collect();
                let count: i32 = split_pull[0].parse().unwrap();
                let color = split_pull[1];

                if count > colors_maxcount[color] {
                    turn_viable = false
                }
            }
        }

        if turn_viable {
            sum = sum + game_id;
        }
    }

    sum.to_string()
}

pub fn part2(reader: BufReader<File>) -> String {
    let mut sum = 0;

    for line in reader.lines() {
        let line_text = line.unwrap();

        let segments: Vec<_> = line_text.split(':').collect();

        let round_segments: Vec<Vec<_>> = segments[1]
            .trim()
            .split(";")
            .map(|x| x.split(", ").map(|x| x.trim()).collect())
            .collect();

        let mut colors_mincount: HashMap<String, i32> = HashMap::new();

        colors_mincount.insert("red".to_string(), 0);
        colors_mincount.insert("green".to_string(), 0);
        colors_mincount.insert("blue".to_string(), 0);

        for turn in round_segments.clone() {
            for pull in turn {
                let split_pull: Vec<_> = pull.split(' ').collect();
                let count: i32 = split_pull[0].parse().unwrap();
                let color = split_pull[1];

                if count > colors_mincount[color] {
                    colors_mincount.insert(color.to_string(), count);
                }
            }
        }

        sum = sum + (colors_mincount["red"] * colors_mincount["green"] * colors_mincount["blue"]);
    }

    sum.to_string()
}
