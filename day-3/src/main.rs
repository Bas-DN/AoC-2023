use std::collections::HashMap;

const _EMPTY: char = '.';
// {'&': [], '-': [], '/': [], '=': [], '%': [], '@': []}
const _SYMBOLS: [char; 10] = ['&', '-', '/', '=', '%', '@', '$', '+', '*', '#'];

fn part1() {
    // Read the file contents
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // let number_locations = get_numbers_with_locations(contents.as_str());
    let adjacent_numbers = get_adjacent_numbers(contents.as_str());
    // println!("{:?}", adjacent_numbers);
    // Sum of the numbers
    let sum: u32 = adjacent_numbers.iter().sum();
    println!("Sum: {}", sum);

}

fn part2() {
    // Read the file contents of sample2.txt
    let contents = std::fs::read_to_string("input.txt").unwrap();
    // Read each line
    let _lines = contents.lines();
    let numbers = get_possible_gear_numbers(contents.as_str());

    // gear ratio is the two numbers multiplied together
    // total gear ratio is the sum of all the gear ratios
    let mut gear_ratios: Vec<u32> = Vec::new();
    for (_key, value) in &numbers {
        if value.len() == 2 {
            gear_ratios.push(value[0] * value[1]);
        }
    }
    let sum: u32 = gear_ratios.iter().sum();
    println!("Sum: {}", sum);
}

fn main() {
    part1();
    part2();
}
fn _find_all_symbol_types(contents: &str) {
    // find the symbols that are not numbers or empty, and put them in a hashmap
    let mut symbols: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for line in contents.lines() {
        for c in line.chars() {
            if !_SYMBOLS.contains(&c) && c != _EMPTY && !c.is_digit(10) {
                symbols.insert(c, Vec::new());
            }
        }
    }
    println!("{:?}", symbols);
    // Check if all the symbols are in _SYMBOLS
    for (symbol, _locations) in &symbols {
        if !_SYMBOLS.contains(symbol) {
            panic!("Symbol {} is not in _SYMBOLS", symbol);
        }
    }
}
// Gear numbers are numbers that are adjacent to a * symbol
// Find the numbers that are adjacent to this symbol (even diagonally)
fn get_possible_gear_numbers(contents: &str) -> HashMap<(usize, usize), Vec<u32>> {
    let mut gear_locations: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    // let mut numbers: Vec<u32> = Vec::new();
    let number_locations = get_numbers_with_locations(contents);
    let symbol_locations = get_symbol_locations(contents);
    // Filter symbols to only include * (gear)
    let symbol_locations: HashMap<char, Vec<(usize, usize)>> = symbol_locations
        .into_iter()
        .filter(|(symbol, _locations)| *symbol == '*')
        .collect();
    for (x, y, number) in number_locations {
        // Check if the number is adjacent to a *
        for (_symbol, locations) in &symbol_locations {
            for (symbol_x, symbol_y) in locations {
                // Check for the number of digits in the number
                let number_of_digits = number_of_digits(number);
                // Check if the symbol is adjacent to the number
                for i in 0..number_of_digits {
                    let x = x + i;
                    if x >= contents.lines().nth(y).unwrap().len() {
                        break;
                    }
                    let c = contents.lines().nth(y).unwrap().chars().nth(x).unwrap();
                    if c == _EMPTY {
                        break;
                    }
                    let has_star = check_locations(symbol_x, x, symbol_y, y);
                    if has_star {
                        gear_locations
                            .entry((*symbol_x, *symbol_y))
                            .or_insert(Vec::new())
                            .push(number);
                        // numbers.push(number);
                        break;
                    }
                }
            }
        }
    }
    gear_locations
}
// Find the numbers that are adjacent to a symbol (even diagonally)
fn get_adjacent_numbers(s: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let number_locations = get_numbers_with_locations(s);
    let symbol_locations = get_symbol_locations(s);
    for (x, y, number) in number_locations {
        // Check if the number is adjacent to a symbol
        for (_symbol, locations) in &symbol_locations {
            for (symbol_x, symbol_y) in locations {
                // Check for the number of digits in the number
                let number_of_digits = number_of_digits(number);
                // Check if the symbol is adjacent to the number
                for i in 0..number_of_digits {
                    let x = x + i;
                    if x >= s.lines().nth(y).unwrap().len() {
                        break;
                    }
                    let c = s.lines().nth(y).unwrap().chars().nth(x).unwrap();
                    if c == _EMPTY {
                        break;
                    }
                    let has_symbol = check_locations(symbol_x, x, symbol_y, y);
                    if has_symbol {
                        numbers.push(number);
                        break;
                    }
                }
            }
        }
    }

    numbers
}

fn check_locations(symbol_x: &usize, x: usize, symbol_y: &usize, y: usize) -> bool {
    let mut has_symbol = false;
    // Check if the symbol is adjacent to the number
    if (*symbol_x as i32 - x as i32).abs() <= 1 && (*symbol_y as i32 - y as i32).abs() <= 1 {
        has_symbol = true;
    }
    // Also check if the symbol is diagonal to the number
    if (*symbol_x as i32 - x as i32).abs() == 1 && (*symbol_y as i32 - y as i32).abs() == 1 {
        has_symbol = true;
    }
    has_symbol
}
// Grab the numbers (which can be multiple digits long, e.g. 467) until we hit a symbol, an empty spot or the end of the line
// Return a vector of the locations of the numbers
fn get_numbers_with_locations(s: &str) -> Vec<(usize, usize, u32)> {
    let mut numbers: Vec<(usize, usize, u32)> = Vec::new();
    for (y, line) in s.lines().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let c = line.chars().nth(x).unwrap();
            if c.is_digit(10) {
                let mut number = c.to_digit(10).unwrap() as u32;
                let mut i = 1;
                while let Some(c) = line.chars().nth(x + i) {
                    if c.is_digit(10) {
                        number *= 10;
                        number += c.to_digit(10).unwrap() as u32;
                        i += 1;
                    } else {
                        break;
                    }
                }
                numbers.push((x, y, number));
                x += i; // Skip the rest of the number
            } else {
                x += 1;
            }
        }
    }
    numbers
}
fn number_of_digits(n: u32) -> usize {
    n.to_string().len()
}
fn get_symbol_locations(s: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // Iterate over each line
    for (y, line) in s.lines().enumerate() {
        // Iterate over each character
        for (x, c) in line.chars().enumerate() {
            // If the character is a symbol, add it to the map
            if _SYMBOLS.contains(&c) {
                // If the symbol is already in the map, add the location to the vector
                if let Some(locations) = map.get_mut(&c) {
                    locations.push((x, y));
                } else {
                    // If the symbol is not in the map, add it to the map
                    map.insert(c, vec![(x, y)]);
                }
            }
        }
    }
    map
}
#[test]
fn test() {
    let map = get_symbol_locations(_TEST_CASE_LINES.join("\n").as_str());
    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&'*').unwrap().len(), 3);
    assert_eq!(map.get(&'+').unwrap().len(), 1);
    assert_eq!(map.get(&'$').unwrap().len(), 1);
}
#[test]
fn test_numbers() {
    let map = get_numbers_with_locations(_TEST_CASE_LINES.join("\n").as_str());
    assert_eq!(map.len(), 10);
}
#[test]
fn test_adjacent_numbers() {
    let numbers = get_adjacent_numbers(_TEST_CASE_LINES.join("\n").as_str());
    assert_eq!(numbers.len(), 8);
    let sum: u32 = numbers.iter().sum();
    assert_eq!(sum, 4361);
}
#[test]
fn test_possible_gear_numbers() {
    let numbers = get_possible_gear_numbers(_TEST_CASE_LINES.join("\n").as_str());
    // Check the amount of u32 in each hashmap, check if there are two hashmaps with 2 u32
    let mut count = 0;
    for (_key, value) in &numbers {
        if value.len() == 2 {
            count += 1;
        }
    }
    assert_eq!(count, 2);
    // gear ratio is the two numbers multiplied together
    // total gear ratio is the sum of all the gear ratios
    let mut gear_ratios: Vec<u32> = Vec::new();
    for (_key, value) in &numbers {
        if value.len() == 2 {
            gear_ratios.push(value[0] * value[1]);
        }
    }
    let sum: u32 = gear_ratios.iter().sum();
    assert_eq!(sum, 467835);
}
const _TEST_CASE_LINES: [&str; 10] = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
];
