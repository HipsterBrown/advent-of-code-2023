fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Part(usize), // (value)
    Symbol,
    Period,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    token: Token,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    points: Vec<Vec<Point>>,
}

impl Grid {
    fn point_has_adjacent_symbol(self: &Self, point: &Point) -> bool {
        let (max_width, max_height) = (self.width - 1, self.height - 1);
        match (point.x, point.y) {
            // top-left corner
            (0, 0) => {
                // check right, bottom, bottom-right
                for (x, y) in [(1, 0), (1, 1), (0, 1)].iter() {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // bottom-right corner
            (x, y) if x == max_width && y == max_height => {
                // check left, top-left, top
                for (x, y) in [
                    (max_width - 1, max_height),
                    (max_width - 1, max_height - 1),
                    (max_width, max_height - 1),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // bottom-left corner
            (0, max_y) if max_y == max_height => {
                // check top, top-right, right
                for (x, y) in [(0, max_height + 1), (1, max_height + 1), (1, max_height)].iter() {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // top-right corner
            (max_x, 0) if max_x == max_width => {
                // check left, bottom-left, bottom
                for (x, y) in [(max_width - 1, 0), (max_width - 1, 1), (max_width, 1)].iter() {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // last column
            (max_x, point_y) if max_x == max_width => {
                // check left, bottom-left, bottom, top-left, top
                for (x, y) in [
                    (max_width - 1, point_y),
                    (max_width - 1, point_y - 1),
                    (max_width, point_y - 1),
                    (max_width - 1, point_y + 1),
                    (max_width, point_y + 1),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // first column
            (0, point_y) => {
                // check top, top-right, right, bottom-right, bottom
                for (x, y) in [
                    (0, point_y - 1),
                    (1, point_y - 1),
                    (1, point_y),
                    (1, point_y + 1),
                    (0, point_y + 1),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // bottom row
            (point_x, max_y) if max_y == max_height => {
                // check left, right, top-left, top, top-right
                for (x, y) in [
                    (point_x - 1, max_height),
                    (point_x + 1, max_height),
                    (point_x - 1, max_height - 1),
                    (point_x, max_height - 1),
                    (point_x + 1, max_height - 1),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // top row
            (point_x, 0) => {
                // check bottom, bottom-left, bottom-right, left, right
                for (x, y) in [
                    (point_x, 1),
                    (point_x - 1, 1),
                    (point_x + 1, 1),
                    (point_x - 1, 0),
                    (point_x + 1, 0),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
            // in-between
            (point_x, point_y) => {
                // check left, top-left, top, top-right, right, bottom-right, bottom, bottom-left
                for (x, y) in [
                    (point_x - 1, point_y),
                    (point_x - 1, point_y - 1),
                    (point_x, point_y - 1),
                    (point_x + 1, point_y - 1),
                    (point_x + 1, point_y),
                    (point_x + 1, point_y + 1),
                    (point_x, point_y + 1),
                    (point_x - 1, point_y + 1),
                ]
                .iter()
                {
                    if self.points[*y][*x].token == Token::Symbol {
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut points: Vec<Vec<Point>> = Vec::new();
    for (y, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        let mut symbols: Vec<Point> = Vec::new();
        let mut current_value = String::new();
        for (x, c) in line.char_indices() {
            match c {
                '.' => {
                    let length = current_value.len();
                    if length > 0 {
                        for n in 0..length {
                            symbols.push(Point {
                                x: x - (length - n),
                                y,
                                token: Token::Part(current_value.parse::<usize>().unwrap()),
                            });
                        }
                        current_value = String::new();
                    }
                    symbols.push(Point {
                        x,
                        y,
                        token: Token::Period,
                    });
                }
                '0'..='9' => {
                    current_value += &c.to_string();
                    if x == line.len() - 1 {
                        let length = current_value.len();
                        for n in 0..length {
                            symbols.push(Point {
                                x: x - (length - n),
                                y,
                                token: Token::Part(current_value.parse::<usize>().unwrap()),
                            });
                        }
                    }
                }
                _ => {
                    let length = current_value.len();
                    if length > 0 {
                        for n in 0..length {
                            symbols.push(Point {
                                x: x - (length - n),
                                y,
                                token: Token::Part(current_value.parse::<usize>().unwrap()),
                            });
                        }
                        current_value = String::new();
                    }
                    symbols.push(Point {
                        x,
                        y,
                        token: Token::Symbol,
                    });
                }
            }
        }
        points.push(symbols);
    }
    let grid = Grid {
        height: points.len(),
        width: points[0].len(),
        points,
    };
    let parts: Vec<&Point> = grid
        .points
        .iter()
        .clone()
        .map(|row| {
            let mut parts = row
                .iter()
                .filter(|point| match point.token {
                    Token::Part(_value) => grid.point_has_adjacent_symbol(point),
                    _ => false,
                })
                .collect::<Vec<&Point>>();
            parts.dedup_by(|a, b| match (a.token, b.token) {
                (Token::Part(a_value), Token::Part(b_value)) => a_value == b_value,
                _ => false,
            });
            parts
        })
        .flatten()
        .collect();

    parts
        .iter()
        .map(|p| match p.token {
            Token::Part(value) => value,
            _ => 0,
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
