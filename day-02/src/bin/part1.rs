fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn part1(input: &str) -> u32 {
    let games = input.lines().filter(|s| !s.is_empty()).map(|line| {
        let mut iter = line.split(": ");
        let id = iter
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let rounds = iter
            .next()
            .unwrap()
            .split("; ")
            .map(|round| {
                let cube_iter = round.split(", ");
                let red = cube_iter
                    .clone()
                    .filter(|c| c.contains("red"))
                    .next()
                    .unwrap_or("0")
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                let blue = cube_iter
                    .clone()
                    .filter(|c| c.contains("blue"))
                    .next()
                    .unwrap_or("0")
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                let green = cube_iter
                    .filter(|c| c.contains("green"))
                    .next()
                    .unwrap_or("0")
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
                println!("red: {red}, blue: {blue}, green: {green}");
                Round { red, blue, green }
            })
            .collect::<Vec<Round>>();
        Game { id, rounds }
    });

    games
        .filter(|game| {
            !game.rounds.iter().any(|round| {
                round.red > RED_LIMIT || round.blue > BLUE_LIMIT || round.green > GREEN_LIMIT
            })
        })
        .map(|g| g.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(result, 8);
    }
}
