use std::fs;

fn find_shared(comp1: &str, comp2: &str) -> char {
    for item in comp1.chars() {
        if comp2.contains(item) {
            return item;
        };
    }

    panic!("No shared found for: \nC1: {}\nC2: {}", comp1, comp2);
}

fn find_badge(sack1: &str, sack2: &str, sack3: &str) -> char {
    for item in sack1.chars() {
        if sack2.contains(item) && sack3.contains(item) {
            return item;
        };
    }

    panic!(
        "No badge found for: \nS1: {}\nS2: {}\nS3: {}",
        sack1, sack2, sack3
    );
}

fn get_prio(item: char) -> u64 {
    let num = item as u64;

    if num > 90 {
        return num - 96;
    } else {
        return (num - 64) + 26;
    }
}

fn part1() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let mut sacks: Vec<&str> = input.split("\n").collect();
    sacks.pop();

    let mut sum: u64 = 0;

    for sack in sacks {
        let comp1: &str = &sack[0..(&sack.len() / 2)];
        let comp2: &str = &sack[&sack.len() / 2..sack.len()];

        let shared = find_shared(comp1, comp2);

        sum += get_prio(shared);
    }

    println!("{}", sum);
}

fn part2() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let mut sacks: Vec<&str> = input.split("\n").collect();
    sacks.pop();

    let groups: Vec<Vec<&str>> = sacks.chunks(3).map(|chunk| chunk.to_vec()).collect();

    let mut sum: u64 = 0;

    for group in groups {
        let badge = find_badge(group[0], group[1], group[2]);

        sum += get_prio(badge);
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
