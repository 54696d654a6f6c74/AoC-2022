use std::fs;

fn get_top_index(col: usize, state: &Vec<&str>) -> usize {
    let target_x = col * 4;

    for row_i in 0..state.len() {
        let row_bytes = state[row_i].as_bytes();

        if row_bytes[target_x] as char == '[' {
            return row_i;
        }
    }

    return 0;
}

fn populate_row(row: &str, stacks: &mut Vec<Vec<char>>) {
    let crates: Vec<String> = row
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    for i in 0..crates.len() {
        let cur_crate = crates[i].as_bytes();

        if cur_crate[0] as char == '[' {
            stacks[i].push(cur_crate[1] as char)
        }
    }
}

fn move_crate_9000(times: u8, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..times {
        let target = stacks[from].pop().unwrap();
        stacks[to].push(target);
    }
}

fn move_crate_9001(times: u8, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    let mut moved_stack = vec![];

    for _ in 0..times {
        let target = stacks[from].pop().unwrap();
        moved_stack.push(target);
    }

    moved_stack.reverse();

    for stack_crate in moved_stack {
        stacks[to].push(stack_crate);
    }
}

fn part1() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut state: Vec<&str> = parts[0].split('\n').collect();

    let state_cols: Vec<&str> = state.pop().unwrap().split("  ").collect();

    state.reverse();

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..state_cols.len() {
        stacks.push(vec![]);
    }

    for row in state {
        populate_row(row, &mut stacks)
    }

    let mut instrictions: Vec<&str> = parts[1].split('\n').collect();

    instrictions.pop();

    for instriction in instrictions {
        let instruction_seg: Vec<&str> = instriction.split(' ').collect();

        let times = instruction_seg[1]
            .parse::<u8>()
            .expect("Failed to parse 'times'");
        let from = instruction_seg[3]
            .parse::<usize>()
            .expect("Failed to parse 'from'");
        let to = instruction_seg[5]
            .parse::<usize>()
            .expect("Failed to parse 'to'");

        move_crate_9000(times, from - 1, to - 1, &mut stacks);
    }

    for mut col in stacks {
        print!("{}", col.pop().unwrap())
    }
    println!();
}

fn part2() {
    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut state: Vec<&str> = parts[0].split('\n').collect();

    let state_cols: Vec<&str> = state.pop().unwrap().split("  ").collect();

    state.reverse();

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..state_cols.len() {
        stacks.push(vec![]);
    }

    for row in state {
        populate_row(row, &mut stacks)
    }

    let mut instrictions: Vec<&str> = parts[1].split('\n').collect();

    instrictions.pop();

    for instriction in instrictions {
        let instruction_seg: Vec<&str> = instriction.split(' ').collect();

        let times = instruction_seg[1]
            .parse::<u8>()
            .expect("Failed to parse 'times'");
        let from = instruction_seg[3]
            .parse::<usize>()
            .expect("Failed to parse 'from'");
        let to = instruction_seg[5]
            .parse::<usize>()
            .expect("Failed to parse 'to'");

        move_crate_9001(times, from - 1, to - 1, &mut stacks);
    }

    for mut col in stacks {
        print!("{}", col.pop().unwrap())
    }

    println!();
}

fn main() {
    part1();
    part2();
}
