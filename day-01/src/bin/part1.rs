fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let total: u32 = lines
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            [first, last]
                .into_iter()
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .inspect(|v: &u32| println!("summing {v}"))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(result, 142);
    }
}
