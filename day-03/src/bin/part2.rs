fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Part {
    col_start: usize,
    col_end: usize,
    row: usize,
    value: usize,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    col: usize,
    row: usize,
    value: char,
}

impl Symbol {
    fn get_adjacent_parts(
        self: &Self,
        max_row: usize,
        max_col: usize,
        parts: &Vec<Part>,
    ) -> Vec<Part> {
        let positions = self.get_possible_positions(max_row, max_col);
        parts
            .iter()
            .filter_map(|part| {
                for position in (part.col_start..=part.col_end).map(|col| (col, part.row)) {
                    if positions.contains(&position) {
                        return Some(*part);
                    }
                }
                None
            })
            .collect()
    }

    fn get_possible_positions(self: &Self, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
        let possible_positions: Vec<(isize, isize)> = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 1),
        ];
        possible_positions
            .iter()
            .filter_map(|(col, row)| {
                if row + (self.row as isize) > max_row as isize || row + (self.row as isize) < 0 {
                    ()
                }
                if col + (self.col as isize) < 0 || col + (self.col as isize) > max_col as isize {
                    ()
                }
                Some((
                    (col + self.col as isize) as usize,
                    (row + self.row as isize) as usize,
                ))
            })
            .collect()
    }
}

fn part2(input: &str) -> usize {
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines[0].len();
    for (row, line) in lines.iter().enumerate() {
        let mut current_value = String::new();
        for (col, c) in line.char_indices() {
            match c {
                '.' => {
                    let length = current_value.len();
                    if length > 0 {
                        parts.push(Part {
                            col_start: col - length,
                            col_end: col - 1,
                            row,
                            value: (current_value.parse::<usize>().unwrap()),
                        });
                        current_value = String::new();
                    }
                }
                '0'..='9' => {
                    current_value += &c.to_string();
                    if col == line.len() - 1 {
                        let length = current_value.len();
                        parts.push(Part {
                            col_start: col - length - 1,
                            col_end: col - 1,
                            row,
                            value: (current_value.parse::<usize>().unwrap()),
                        });
                    }
                }
                _ => {
                    let length = current_value.len();
                    if length > 0 {
                        parts.push(Part {
                            col_start: col - length,
                            col_end: col - 1,
                            row,
                            value: (current_value.parse::<usize>().unwrap()),
                        });
                        current_value = String::new();
                    }
                    symbols.push(Symbol { col, row, value: c })
                }
            }
        }
    }
    let gear_ratios = symbols.iter().filter_map(|symbol| {
        if symbol.value != '*' {
            return None;
        }
        let adjacent_parts = symbol.get_adjacent_parts(height, width, &parts);
        if adjacent_parts.iter().len() < 2 {
            return None;
        }
        Some(
            adjacent_parts
                .iter()
                .map(|part| part.value)
                .product::<usize>(),
        )
    });
    gear_ratios.sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(result, 467835);
    }
}
