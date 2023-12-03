fn day1p1() {
    // Read the file contents
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // Read each line
    let lines = contents.lines();
    // List of numbers
    let mut row_numbers: Vec<i32> = Vec::new();
    for line in lines {
        // println!("{}", line);
        // Select all numbers from the line of characters
        let numbers: Vec<i32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as i32)
            .collect();
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        // combine the first and last number (e.g. 1 and 2 becomes 12)
        let combined_number = format!("{}{}", first, last);
        let combined_number = combined_number.parse::<i32>().unwrap();
        row_numbers.push(combined_number);
    }
    // Sum all the numbers
    let sum: i32 = row_numbers.iter().sum();
    println!("Sum: {}", sum);
}

fn day1p2() {
    // Read the file contents of sample2.txt
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // Read each line
    let lines = contents.lines();
    // List of numbers
    let mut row_numbers: Vec<i32> = Vec::new();
    for line in lines {
        // Select all numbers from the line of characters
        // Find all the numbers or words that are numbers and store them in a vector
        let numbers: Vec<i32> = extract_numbers(line);

        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        // combine the first and last number (e.g. 1 and 2 becomes 12)
        let combined_number = format!("{}{}", first, last);
        let combined_number = combined_number.parse::<i32>().unwrap();
        row_numbers.push(combined_number);
    }
    let sum: i32 = row_numbers.iter().sum();
    println!("Sum: {}", sum);
}

fn main() {
    day1p1();
    day1p2();
}

fn extract_numbers(s: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut word = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_digit(10) {
            find_number_word(&mut word, &mut numbers);
            numbers.push(ch.to_digit(10).unwrap() as i32);
        } else if ch.is_alphabetic() {
            word.push(ch);
            // If it's at the end of the string, try to parse the word as one or multiple numbers
            if i == s.len() - 1 {
                find_number_word(&mut word, &mut numbers);
            }
        }
    }
    numbers
}
// some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
fn find_number_word(word: &mut String, numbers: &mut Vec<i32>) {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    while !word.is_empty() {
        for (number_word, number) in number_words.iter() {
            if word.starts_with(number_word) {
                numbers.push(*number);
                break;
            }
        }
        word.remove(0);
    }
}
#[test]
fn test_extract_numbers() {
    let numbers: Vec<i32> = extract_numbers("two1nine");
    assert_eq!(numbers, vec![2, 1, 9]);
}
#[test]
fn test_extract_numbers2() {
    let numbers2: Vec<i32> = extract_numbers("xtwone3four");
    assert_eq!(numbers2, vec![2, 1, 3, 4]);
}
#[test]
fn test_extract_numbers3() {
    let numbers3: Vec<i32> = extract_numbers("zoneight234");
    assert_eq!(numbers3, vec![1, 8, 2, 3, 4]);
}
