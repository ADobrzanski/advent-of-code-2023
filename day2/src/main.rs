use regex::Regex;
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    index: i32,
    draws: Vec<Draw>,
}

fn main() {
    let input =
        std::fs::read_to_string("./input.txt").expect("Could not read from \"./input.txt\"");

    let result = input
        .lines()
        .map(|line| {
            let (header, draws_raw) = line.split_once(": ").expect("Could not parse line");

            let index = header[5..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<i32>()
                .expect("Could not parse index");

            /* Regex that captures digits before word "red" */
            let regex_red = Regex::new(r"(\d+) red").unwrap();
            let regex_blue = Regex::new(r"(\d+) blue").unwrap();
            let regex_green = Regex::new(r"(\d+) green").unwrap();

            let draws = draws_raw
                .split("; ")
                .map(|draw_raw| {
                    let mut draw = Draw {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };

                    for cap in regex_red.captures_iter(draw_raw) {
                        draw.red = cap[1].parse::<i32>().unwrap();
                    }

                    for cap in regex_green.captures_iter(draw_raw) {
                        draw.green = cap[1].parse::<i32>().unwrap();
                    }

                    for cap in regex_blue.captures_iter(draw_raw) {
                        draw.blue = cap[1].parse::<i32>().unwrap();
                    }

                    draw
                })
                .collect();

            Game { index, draws }
        })
        /* PART 1 */
        // .filter(|game| {
        //     game.draws
        //         .iter()
        //         .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        // })
        // .map(|game| game.index)
        /* PART 2 */
        .map(|game| {
            let max_draw = Draw {
                red: game.draws.iter().map(|draw| draw.red).max().unwrap_or(0),
                green: game.draws.iter().map(|draw| draw.green).max().unwrap_or(0),
                blue: game.draws.iter().map(|draw| draw.blue).max().unwrap_or(0),
            };

            max_draw.red * max_draw.green * max_draw.blue
        })
        .sum::<i32>();

    println!("Result: {}", result);
}
