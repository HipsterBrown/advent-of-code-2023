fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

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

fn part2(input: &str) -> u32 {
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
                Round { red, blue, green }
            })
            .collect::<Vec<Round>>();
        Game { id, rounds }
    });

    let min_games = games.map(|game| {
        // (red, blue, green)
        let mut min_cubes = (0, 0, 0);
        for round in game.rounds.iter() {
            if round.red > min_cubes.0 {
                min_cubes.0 = round.red;
            }
            if round.blue > min_cubes.1 {
                min_cubes.1 = round.blue;
            }
            if round.green > min_cubes.2 {
                min_cubes.2 = round.green;
            }
        }
        min_cubes.0 * min_cubes.1 * min_cubes.2
    });

    min_games.sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(result, 2286);
    }
}
