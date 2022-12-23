use std::fs;

fn part1() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let elves = input.split("\n\n");

    let mut sums = vec!();

    for elf in elves {
        let elf_calories = String::from(elf);

        let mut sum = 0;

        for calory in elf_calories.split('\n') {
            let parsed_calory = String::from(calory).parse::<i32>().unwrap();
            sum += parsed_calory;
        }

        sums.push(sum);
    }

    println!("{}", sums.iter().max().unwrap());
}

fn part2() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let elves = input.split("\n\n");

    let mut sums = vec!();

    for elf in elves {
        let elf_calories = String::from(elf);

        let mut sum = 0;

        for calory in elf_calories.split('\n') {
            let parsed_calory = String::from(calory).parse::<i32>().unwrap();
            sum += parsed_calory;
        }

        sums.push(sum);
    }

    sums.sort();

    println!("{}", sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]);
}

fn main() {
    part1();
    part2();
}
