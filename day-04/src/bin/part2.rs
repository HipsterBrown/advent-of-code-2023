use std::collections::btree_map::BTreeMap;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
* store card number to add with number of matches to determine the copies
* create a map of card numbers and number of matches
* create a different map of card number and number of copies to increment while iterating over each
* card's matches for each copy of the current card
**/

fn part2(input: &str) -> usize {
    let mut cards: BTreeMap<usize, (Vec<usize>, Vec<usize>)> = BTreeMap::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut card_iter = line.split(": ");
        let card_id = card_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut numbers = card_iter.next().unwrap().split(" | ").map(|nums_str| {
            nums_str
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        });
        cards.insert(card_id, (numbers.next().unwrap(), numbers.next().unwrap()));
    }

    let mut card_copies: BTreeMap<usize, usize> = BTreeMap::new();
    for (card_id, (winning_numbers, played_numbers)) in cards.iter() {
        let copy_count = card_copies.entry(*card_id).or_insert(1);
        let match_count: usize = played_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .count();
        println!("card_id {card_id}, match_count {match_count}, copy_count {copy_count}");
        if match_count > 0 {
            for _ in 0..*copy_count {
                for copy in 1..=match_count {
                    card_copies.entry(card_id + copy).or_insert(1);
                    card_copies.entry(card_id + copy).and_modify(|count| {
                        *count += 1;
                    });
                }
            }
        }
    }
    card_copies.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(result, 30);
    }
}
