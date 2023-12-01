fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let valid_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let lines = input.lines();
    let total: u32 = lines
        .map(|line| {
            let mut digits: Vec<(usize, &str)> = valid_digits
                .into_iter()
                .map(|digit| line.match_indices(digit).filter(|s| !s.1.is_empty()))
                .flatten()
                .collect();
            digits.sort_by(|a, b| a.0.cmp(&b.0));

            let mut digits_iter = digits.iter();
            let first = digits_iter.next().unwrap();
            let last = digits_iter.last().unwrap_or(first);
            [first, last]
                .into_iter()
                .map(|d| match d {
                    (_i, "one") | (_i, "1") => "1",
                    (_i, "two") | (_i, "2") => "2",
                    (_i, "three") | (_i, "3") => "3",
                    (_i, "four") | (_i, "4") => "4",
                    (_i, "five") | (_i, "5") => "5",
                    (_i, "six") | (_i, "6") => "6",
                    (_i, "seven") | (_i, "7") => "7",
                    (_i, "eight") | (_i, "8") => "8",
                    (_i, "nine") | (_i, "9") => "9",
                    _ => panic!("Found an unknown value {:#?}", d),
                })
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    total
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\nnine8foursnczninednds");
        assert_eq!(result, 380);
    }
}
