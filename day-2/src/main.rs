use std::collections::HashMap;

fn part1() {
    // Read the file contents
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // Read each line
    let lines = contents.lines();
    let mut max: HashMap<&str, i32> = HashMap::new();
    max.insert("red", 12);
    max.insert("green", 13);
    max.insert("blue", 14);
    let mut sum = 0;
    // List of numbers
    for line in lines {
        sum += validate_max_game(line, max.clone());
    }
    println!("Sum: {}", sum);
}

fn part2() {
    // Read the file contents of sample2.txt
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // Read each line
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        sum += validate_min_game(line);
    }
    println!("Sum: {}", sum);
}
fn main() {
    part1();
    part2();
}
const TYPES: [&str; 3] = ["red", "green", "blue"];
const GRAB_SEPERATOR: &str = ";";
fn count_cubes(s: &str) -> HashMap<&&str, i32> {
    // A map with the number of cubes of each type
    let mut map = HashMap::new();
    // For each game (which is a line starting with "Game ID: ")
    for grab in s.split(GRAB_SEPERATOR) {
        // For each type
        for t in TYPES.iter() {
            // Find the number of cubes of this type
            let mut count = 0;
            for c in grab.split(",") {
                if c.contains(t) {
                    count += c.split_whitespace().next().unwrap().parse::<i32>().unwrap();
                }
            }
            // Add the number of cubes of this type to the map
            // Only add if the count is higher than the current count
            if let Some(current_count) = map.get(t) {
                if count > *current_count {
                    map.insert(t, count);
                }
            } else {
                map.insert(t, count);
            }
        }
    }
    // Return the map
    map
}

// Check if the game is possible based on the provided maximum number of cubes of each type
// Return the sum of the ids of the possible games
fn validate_max_game(s: &str, values_to_check: HashMap<&str, i32>) -> i32 {
    const GAME_SEPERATOR: &str = ":";
    // Select the id (the number between "Game " and ":")
    let id = s
        .split(GAME_SEPERATOR)
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    // Select the grabs (the part after the ":")
    let s = s.split(GAME_SEPERATOR).nth(1).unwrap();

    // Count the cubes
    let map = count_cubes(s);
    // For each type
    for (t, count) in map.iter() {
        // Check if the count is higher than the maximum
        if let Some(max_count) = values_to_check.get(*t) {
            if count > max_count {
                // If so, return 0
                return 0;
            }
        }
    }
    // If all counts are lower than the maximum, return the id
    id
}

fn validate_min_game(s: &str) -> i32 {
    const GAME_SEPERATOR: &str = ":";
    // Select the grabs (the part after the ":")
    let s = s.split(GAME_SEPERATOR).nth(1).unwrap();
    // Count the cubes
    let map = count_cubes(s);
    // Multiply the number of cubes with each other
    let mut product = 1;
    for (_, count) in map.iter() {
        if *count > 0 && *count != i32::MAX {
            product *= count;
        }
    }
    // Return the product
    product
}

const _TEST_CASES: [&str; 5] = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
];
#[test]
fn test_validate_game() {
    let mut max: HashMap<&str, i32> = HashMap::new();
    max.insert("red", 12);
    max.insert("green", 13);
    max.insert("blue", 14);
    // Base test results
    let test_results = vec![1, 2, 0, 0, 5];
    // For each test case
    for (i, test_case) in _TEST_CASES.iter().enumerate() {
        // Validate the game
        let result = validate_max_game(test_case, max.clone());
        // Check if the result is correct
        assert_eq!(result, test_results[i]);
    }
}
#[test]
fn test_validate_game_fewest() {
    // Base test results
    let test_results = vec![48, 12, 1560, 630, 36];
    // For each test case
    for (i, test_case) in _TEST_CASES.iter().enumerate() {
        // Validate the game
        let result = validate_min_game(test_case);
        // Check if the result is correct
        assert_eq!(result, test_results[i]);
    }
}
