use std::fs;
use std::collections::HashMap;

fn part1() {
    let mut game_options = HashMap::new();

    game_options.insert('X', 1);
    game_options.insert('Y', 2);
    game_options.insert('Z', 3);

    let mut rock_table = HashMap::new();

    rock_table.insert('A', 3);
    rock_table.insert('B', 0);
    rock_table.insert('C', 6);

    let mut paper_table = HashMap::new();

    paper_table.insert('A', 6);
    paper_table.insert('B', 3);
    paper_table.insert('C', 0);


    let mut scissor_table = HashMap::new();

    scissor_table.insert('A', 0);
    scissor_table.insert('B', 6);
    scissor_table.insert('C', 3);

    let mut game_table = HashMap::new();

    game_table.insert('X', rock_table);
    game_table.insert('Y', paper_table);
    game_table.insert('Z', scissor_table);

    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let games = input.split("\n");

    let mut score = 0;

    for game in games {
        let my_shape = &game.chars().nth(2).expect("Failed to get my shape");
        let their_shape = &game.chars().nth(0).expect("Failed to get their shape");

        let game_score = game_table.get(my_shape).unwrap().get(their_shape).unwrap();
        let selecteion_score = game_options.get(my_shape).expect("Failed to get selection score");

        score += game_score + selecteion_score;
    }

    println!("{}", score);
}

fn part2() {
    let mut game_results = HashMap::new();

    game_results.insert('X', 0);
    game_results.insert('Y', 3);
    game_results.insert('Z', 6);

    let mut rock_table = HashMap::new();

    rock_table.insert('X', 3);
    rock_table.insert('Y', 1);
    rock_table.insert('Z', 2);

    let mut paper_table = HashMap::new();

    paper_table.insert('X', 1);
    paper_table.insert('Y', 2);
    paper_table.insert('Z', 3);


    let mut scissor_table = HashMap::new();

    scissor_table.insert('X', 2);
    scissor_table.insert('Y', 3);
    scissor_table.insert('Z', 1);

    let mut game_table = HashMap::new();

    game_table.insert('A', rock_table);
    game_table.insert('B', paper_table);
    game_table.insert('C', scissor_table);

    let input = fs::read_to_string("./inputFile").expect("Kill me :(");
    let games = input.split("\n");

    let mut score = 0;

    for game in games {
        let my_action = &game.chars().nth(2).expect("Failed to get my shape");
        let their_shape = &game.chars().nth(0).expect("Failed to get their shape");

        let game_score = game_results.get(my_action).expect("Failed to get game score");
        let selecteion_score = game_table.get(their_shape).unwrap().get(my_action).expect("Failed to get selection score");

        score += game_score + selecteion_score;
    }

    println!("{}", score);
}

fn main() {
    part1();
    part2();
}
