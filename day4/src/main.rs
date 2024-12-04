use std::{collections::HashSet, io::BufRead};

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    }
}

#[derive(Debug)]
struct WordSearchMatrix {
    matrix: Vec<Vec<char>>,
}

impl WordSearchMatrix {
    fn new() -> Self {
        WordSearchMatrix { matrix: Vec::new() }
    }

    fn add_row(&mut self, row: String) {
        self.matrix
            .push(row.chars().filter(char::is_ascii_alphabetic).collect());
    }

    fn get_search_line(&self, x_dir: i8, y_dir: i8) -> Vec<WordSearchLine> {
        let mut lines = Vec::new();
        let x_bounds = 0..self.matrix[0].len();
        let y_bounds = 0..self.matrix.len();

        // Iterate over the edges
        if x_dir != 0 {
            for start_y in y_bounds.clone() {
                let mut line = Vec::new();
                let mut x = if x_dir == 1 {
                    0
                } else {
                    self.matrix[0].len() - 1
                } as i64;
                let mut y = start_y as i64;
                let ident = WordSearchLineIdent {
                    x: x as usize,
                    y: y as usize,
                    x_dir,
                    y_dir,
                };

                while x_bounds.contains(&(x as usize)) && y_bounds.contains(&(y as usize)) {
                    line.push(&self.matrix[y as usize][x as usize]);
                    x += x_dir as i64;
                    y += y_dir as i64;
                }

                if !line.is_empty() {
                    lines.push(WordSearchLine::new(line, ident));
                }
            }
        }

        if y_dir != 0 {
            for start_x in x_bounds.clone() {
                let mut line = Vec::new();
                let mut x = start_x as i64;
                let mut y = if y_dir == 1 { 0 } else { self.matrix.len() - 1 } as i64;
                let ident = WordSearchLineIdent {
                    x: x as usize,
                    y: y as usize,
                    x_dir,
                    y_dir,
                };

                while x_bounds.contains(&(x as usize)) && y_bounds.contains(&(y as usize)) {
                    line.push(&self.matrix[y as usize][x as usize]);
                    x += x_dir as i64;
                    y += y_dir as i64;
                }

                if !line.is_empty() {
                    lines.push(WordSearchLine::new(line, ident));
                }
            }
        }
        lines
    }

    fn get_search_lines(&self) -> Vec<WordSearchLine> {
        [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .into_iter()
        .map(|(x, y)| self.get_search_line(x, y))
        .flatten()
        .collect()
    }

    fn get_sub_grids(&self) -> Vec<XSubGrid> {
        let mut sub_grids = Vec::new();
        for y in 0..self.matrix.len() - 2 {
            for x in 0..self.matrix[0].len() - 2 {
                let grid = [
                    [&self.matrix[y][x], &self.matrix[y][x + 1], &self.matrix[y][x + 2]],
                    [&self.matrix[y + 1][x], &self.matrix[y + 1][x + 1], &self.matrix[y + 1][x + 2]],
                    [&self.matrix[y + 2][x], &self.matrix[y + 2][x + 1], &self.matrix[y + 2][x + 2]],
                ];
                sub_grids.push(XSubGrid { grid });
            }
        }
        sub_grids
    }
}

struct XSubGrid<'a> {
    grid: [[&'a char; 3]; 3],
}

impl XSubGrid<'_> {
    fn is_x(&self) -> bool {
        self.is_cross_diag_right() && self.is_diag_right()
    }
    fn is_diag_right(&self) -> bool {
        match self.grid {
            [[a, _, _], [_, b, _], [_, _, c]] => (a == &'M' && b == &'A' && c == &'S') || (a == &'S' && b == &'A' && c == &'M'),
        }
    }

    fn is_cross_diag_right(&self) -> bool {
        match self.grid {
            [[_, _, a], [_, b, _], [c, _, _]] => (a == &'M' && b == &'A' && c == &'S') || (a == &'S' && b == &'A' && c == &'M'),
        }
    }
}

#[derive(Debug, Clone)]
struct WordSearchLineIdent {
    x: usize,
    y: usize,
    x_dir: i8,
    y_dir: i8,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FoundIdent {
    x: usize,
    y: usize,
    x_dir: i8,
    y_dir: i8,
}

impl WordSearchLineIdent {
    fn ident(&self, i: i32) -> FoundIdent {
        FoundIdent {
            x: (self.x as i32 + i * self.x_dir as i32) as usize,
            y: (self.y as i32 + i * self.y_dir as i32) as usize,
            x_dir: self.x_dir,
            y_dir: self.y_dir,
        }
    }
}

#[derive(Debug)]
struct WordSearchLine<'a> {
    ident: WordSearchLineIdent,
    line: Vec<&'a char>,
}

impl<'a> WordSearchLine<'a> {
    fn new(line: Vec<&'a char>, ident: WordSearchLineIdent) -> Self {
        WordSearchLine { line, ident }
    }

    fn find_xmas_ident(&self) -> Vec<FoundIdent> {
        if self.line.len() < 4 {
            return vec![];
        }
        let mut found = vec![];
        let mut i = 0;
        while i < self.line.len() - 3 {
            match self.line[i..i + 4] {
                [a, b, c, d] => {
                    if a == &'X' && b == &'M' && c == &'A' && d == &'S' {
                        found.push(self.ident.ident(i as i32));
                        i += 3;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        found
    }
}

fn main() {
    let mut matrix = WordSearchMatrix::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            matrix.add_row(line.unwrap());
        }
    }
    // dprintln!("input: {:?}", matrix);

    let search_lines = matrix.get_search_lines();
    dprintln!("search_lines: {:?}", search_lines.iter().count());
    let result = search_lines
        .iter()
        .map(|line| line.find_xmas_ident())
        .flatten()
        .collect::<HashSet<_>>();
    println!("Result: {}", result.len());

    let sub_grids = matrix.get_sub_grids();
    let result = sub_grids.iter().filter(|grid| grid.is_x()).count();

    println!("Result: {}", result);
}
