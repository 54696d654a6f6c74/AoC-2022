use std::fs;

fn contains_dups(set: &[char]) -> bool {
    println!("{:?}", set);

    for (i, target) in set.iter().enumerate() {
        for (k, item) in set.iter().enumerate() {
            if target == item && i != k {
                return true;
            }
        }
    }

    return false;
}

fn part1() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");

    let signal = input.chars().collect::<Vec<char>>();

    let mut first_index: i32 = -1;

    for (i, _) in signal.iter().enumerate() {
        if i < 4 {
            continue;
        }

        if !contains_dups(&signal[i - 4..i]) {
            first_index = i as i32;
            break;
        }
    }

    println!("{}", first_index);
}

fn part2() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");

    let signal = input.chars().collect::<Vec<char>>();

    let mut first_index: i32 = -1;

    for (i, _) in signal.iter().enumerate() {
        if i < 14 {
            continue;
        }

        if !contains_dups(&signal[i - 14..i]) {
            first_index = i as i32;
            break;
        }
    }

    println!("{}", first_index);
}

fn main() {
    part1();
    part2();
}
