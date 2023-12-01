fn word_to_digit(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => word,
    }
}

fn main() {
    let input =
        std::fs::read_to_string("./input.txt").expect("Could not read from \"./input.txt\"");

    let digits_strings = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let result = input
        .lines()
        .map(|line| {
            let mut digits: Vec<&str> = Vec::new();

            for offset in 0..line.chars().count() {
                if let Some(digit_word) = digits_strings
                    .into_iter()
                    .find(|word| line[offset..].starts_with(word))
                {
                    digits.push(word_to_digit(digit_word));
                }
            }

            digits.join("")
        })
        .map(|line| {
            let first_digit_char = line
                .chars()
                .find(|char| char.is_numeric())
                .expect("Could not find digit in line");

            let last_digit_char = line
                .chars()
                .rev()
                .find(|char| char.is_numeric())
                .expect("Could not find digit in line");

            let mut calibration_value = first_digit_char.to_string();
            calibration_value.push(last_digit_char);
            calibration_value
                .parse::<i32>()
                .expect("Could not parse number")
        })
        .sum::<i32>();

    println!("Result: {}", result);
}
