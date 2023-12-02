use std::{fs::File, io::BufReader, time::Instant};

mod day1;
mod day2;

fn main() {
    measure_solution("Day 1 p1:".to_owned(), "src/inputs/day1.txt", day1::part1);
    measure_solution("Day 1 p2:".to_owned(), "src/inputs/day1.txt", day1::part2);
    measure_solution("Day 2 p1:".to_owned(), "src/inputs/day2.txt", day2::part1);
    measure_solution("Day 2 p2:".to_owned(), "src/inputs/day2.txt", day2::part2);
}

fn measure_solution(message: String, input: &str, f: fn(file: BufReader<File>) -> String) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let start_time = Instant::now();
    let result = f(reader);
    let elapsed_time = start_time.elapsed();

    // Print the result and elapsed time
    println!("{}", message);
    println!("Function result: {:?}", result);
    println!("Elapsed time: {:?}", elapsed_time);
}
