use std::fs;

fn part1() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let mut assignments: Vec<&str> = input.split('\n').collect();
    assignments.pop();

    let mut sum: u32 = 0;

    for assignment_pair in assignments {
        let elves: Vec<&str> = assignment_pair.split(',').collect();

        let elf1: Vec<u16> = elves[0]
            .split('-')
            .map(|s| s.parse().expect("Shoot me"))
            .collect();
        let elf2: Vec<u16> = elves[1]
            .split('-')
            .map(|s| s.parse().expect("Shoot me"))
            .collect();

        if (elf1[0] <= elf2[0] && elf1[1] >= elf2[1]) || (elf2[0] <= elf1[0] && elf2[1] >= elf1[1])
        {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let mut assignments: Vec<&str> = input.split('\n').collect();
    assignments.pop();

    let mut sum: u32 = 0;

    for assignment_pair in assignments {
        let elves: Vec<&str> = assignment_pair.split(',').collect();

        let elf1: Vec<u16> = elves[0]
            .split('-')
            .map(|s| s.parse().expect("Shoot me"))
            .collect();
        let elf2: Vec<u16> = elves[1]
            .split('-')
            .map(|s| s.parse().expect("Shoot me"))
            .collect();

        if (elf1[1] >= elf2[0]) && (elf2[1] >= elf1[0]) {
            sum += 1
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
