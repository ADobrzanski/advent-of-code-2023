use std::ops::{Range, Sub};

use regex::Regex;

fn find_with_coordinates<'a>(regex: &Regex, input: &'a str) -> Vec<(usize, Range<usize>, &'a str)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_no, line)| {
            regex
                .captures_iter(line)
                .filter_map(|captures| captures.get(0))
                .map(move |matched| (line_no, matched.range(), matched.as_str()))
        })
        .collect::<Vec<(usize, Range<usize>, &str)>>()
}

fn find_neighbours<'a>(
    element: &(usize, Range<usize>, &str),
    neighbourhood: &'a Vec<(usize, Range<usize>, &'a str)>,
) -> Vec<&'a (usize, Range<usize>, &'a str)> {
    let vertical_neighbourhood = element.0.checked_sub(1).unwrap_or(0)..=element.0 + 1;
    let horizontal_neighbourhood = element.1.start.checked_sub(1).unwrap_or(0)..=element.1.end;

    neighbourhood
        .into_iter()
        .filter(|(neighbour_y, neighbour_x_range, _)| {
            vertical_neighbourhood.contains(&neighbour_y)
                && (horizontal_neighbourhood.contains(&neighbour_x_range.start)
                    || horizontal_neighbourhood
                        .contains(&neighbour_x_range.end.checked_sub(1).unwrap_or(0)))
        })
        .collect::<Vec<&'a (usize, Range<usize>, &str)>>()
}

fn main() {
    let input =
        std::fs::read_to_string("./input.txt").expect("Could not read from \"./input.txt\"");

    let part_number_regex = Regex::new(r"\d+").expect("Could not create part number Regex");
    let part_numbers = find_with_coordinates(&part_number_regex, &input);

    let special_character_regex =
        Regex::new(r"[^\w\s.]").expect("Could not create special character Regex");
    let special_characters = find_with_coordinates(&special_character_regex, &input);

    let part1 = part_numbers
        .iter()
        .filter(|part| find_neighbours(&part, &special_characters).len() > 0)
        .map(|part_number| {
            part_number
                .2
                .parse::<i32>()
                .expect("Could not parse part number")
        })
        .sum::<i32>();

    println!("Part1: {}", part1);

    let gears = special_characters
        .iter()
        .filter(|char| char.2.eq("*"))
        .collect::<Vec<&(usize, Range<usize>, &str)>>();

    let part2 = gears
        .iter()
        .filter_map(|gear| {
            let parts = find_neighbours(&gear, &part_numbers);
            if parts.len() == 2 {
                let adjacent_part1 = parts[0].2.parse::<i32>().expect("Could not parse");
                let adjacent_part2 = parts[1].2.parse::<i32>().expect("Could not parse");
                Some(adjacent_part1 * adjacent_part2)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("Part2: {}", part2);
}
