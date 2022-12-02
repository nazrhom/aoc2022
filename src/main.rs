use std::fs;

fn main() {
    day1();
}

fn day1() {
    let contents = fs::read_to_string("./inputs/day1.1.txt")
        .expect("Unable to read file");
    let elves = contents.split("\n\n");
    let mut top_elves: Vec<i32> = elves
                        .map(|e| { e.lines().map(|i| { i.parse::<i32>().unwrap() }).sum::<i32>() })
                        .collect();
    top_elves.sort();
    top_elves.reverse();
    let top3: Vec<&i32> = top_elves.iter().take(3).collect();
    print!("max {}", top3[0]);
    let sum: i32 = top3.into_iter().sum();
    print!("sum {:?}", sum);
}

fn day2() {
    let contents = fs::read_to_string("./inputs/day2.1.txt");
}