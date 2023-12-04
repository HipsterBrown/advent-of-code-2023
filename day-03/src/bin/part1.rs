fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Part {
    col_start: usize,
    col_end: usize,
    row: usize,
    value: usize,
}

impl Part {
    fn has_adjacent_symbol(
        self: &Self,
        max_row: usize,
        max_col: usize,
        symbols: &Vec<Symbol>,
    ) -> bool {
        let positions = self.get_possible_positions(max_row, max_col);
        println!("{:#?}", positions);

        for symbol in symbols.iter() {
            if positions.contains(&(symbol.col, symbol.row)) {
                return true;
            }
        }
        false
    }

    fn get_possible_positions(self: &Self, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
        println!("get_possible_positions");
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
        let part_positions = (self.col_start..=self.col_end)
            .map(|col| (col as isize, self.row as isize))
            .collect::<Vec<(isize, isize)>>();
        println!("part positions: {:#?}", part_positions);
        possible_positions
            .iter()
            .filter_map(|(col, row)| {
                if row + (self.row as isize) > max_row as isize || row + (self.row as isize) < 0 {
                    ()
                }
                if col + (self.col_start as isize) < 0
                    || col + (self.col_end as isize) > max_col as isize
                {
                    ()
                }
                Some(
                    part_positions
                        .iter()
                        .map(|(part_col, part_row)| {
                            ((part_col + col) as usize, (part_row + row) as usize)
                        })
                        .collect::<Vec<(usize, usize)>>(),
                )
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    col: usize,
    row: usize,
    value: char,
}

fn part1(input: &str) -> usize {
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
    let valid_parts = parts
        .iter()
        .filter(|part| part.has_adjacent_symbol(height, width, &symbols));
    valid_parts.map(|p| p.value).sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
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
        assert_eq!(result, 4361);
    }
}
