fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut numbers = line
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|nums_str| {
                    nums_str
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|num_str| num_str.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                });
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .filter_map(|(winning_numbers, potential_matches)| {
            let matches: Vec<&usize> = potential_matches
                .iter()
                .filter(|num| winning_numbers.contains(num))
                .collect();
            if matches.len() == 0 {
                return None;
            }
            Some(2usize.pow((matches.len() - 1) as u32))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(result, 13);
    }
}
