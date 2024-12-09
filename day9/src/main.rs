use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    File(i32),
}

impl Block {
    fn is_empty(&self) -> bool {
        match self {
            Block::Empty => true,
            _ => false,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            Block::File(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::Empty => write!(f, "."),
            Block::File(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: i32,
    size: i8,
}

impl File {
    fn as_block(&self) -> Block {
        Block::File(self.id)
    }
}

#[derive(Debug, Clone)]
struct FS {
    layout: Vec<Block>,
    files: HashMap<usize, File>,
}

impl FS {
    fn new() -> FS {
        FS {
            layout: Vec::new(),
            files: HashMap::new(),
        }
    }

    fn parse(&mut self, line: &str) {
        if self.layout.len() != 0 {
            panic!("FS::parse called on non-empty FS");
        }
        let input = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as i8);
        let mut processed_files = HashSet::new();
        self.layout.extend(
            input
                .enumerate()
                .map(|(i, s)| {
                    (0..s).map(move |_| match i % 2 {
                        0 => (s, Block::File((i as i32) / 2)),
                        1 => (s, Block::Empty),
                        _ => unreachable!(),
                    })
                })
                .flatten()
                .enumerate()
                .inspect(|(i, (s, b))| {
                    if let Block::File(id) = b {
                        if !processed_files.contains(id) {
                            processed_files.insert(*id);
                            self.files.insert(*i as usize, File { id: *id, size: *s });
                        }
                    }
                })
                .map(|(_, (_, b))| b)
        );
    }

    fn moved(&self) -> Self {
        let mut fs = self.clone();

        fn find(fs: &FS, look_for_empty: bool) -> (usize, &Block) {
            fs.layout
                .iter()
                .enumerate()
                .find(|(_, b)| {
                    if look_for_empty {
                        b.is_empty()
                    } else {
                        b.is_file()
                    }
                })
                .unwrap()
        }

        // fn rfind(fs: &FS, look_for_empty: bool) -> (usize, &Block) {
        //     fs.layout
        //         .iter()
        //         .enumerate()
        //         .rfind(|(_, b)| {
        //             if look_for_empty {
        //                 b.is_empty()
        //             } else {
        //                 b.is_file()
        //             }
        //         })
        //         .unwrap()
        // }

        // while rfind(&fs, true).0 != find(&fs, true).0 {
        //     let (i_file, file) = rfind(&fs, false);
        //     let (i_empty, _) = find(&fs, true);
        //     fs.layout.swap(i_file, i_empty);
        //     println!("{}", fs);
        // }
        let mut done = false;
        let mut i = 0;
        self.layout.iter().enumerate().rev().for_each(|(ir, b)| {
            if done {
                return;
            }
            i = find(&fs, true).0;
            if i >= ir {
                done = true;
                return;
            }
            if b.is_file() {
                fs.layout[i] = *b;
                fs.layout[ir] = Block::Empty;
            }
        });

        fs
    }

    fn file_moved(&self) -> Self {
        let mut fs = self.clone();

        fn find_space(fs: &FS, size: i8) -> Option<usize> {
            fs.layout
                .windows(size as usize)
                .enumerate()
                .find_map(|(i, w)| {
                    if w.iter().all(|b| b.is_empty()) {
                        Some(i)
                    } else {
                        None
                    }
                })
        }

        let mut to_move = Vec::new();
        to_move.extend(self.files.iter());
        to_move.sort_by_key(|(_, f)| f.id);

        let mut i = 0;

        to_move.iter().rev().for_each(|(ir, f)| {
            if let Some(i_f) = find_space(&fs, f.size) {
                i = i_f;
            } else {
                // println!("FF no space for {:?}", f);
                return;
            }
            let ir = **ir;
            if i >= ir {
                // done = true;
                // println!("FF done {} >= {}", i, ir);
                return;
            }
            // println!("FF moving {} to {}", ir, i);
            fs.layout[i..i + f.size as usize]
                .iter_mut()
                .for_each(|b| *b = f.as_block());
            fs.layout[ir..ir + f.size as usize]
                .iter_mut()
                .for_each(|b| *b = Block::Empty);

            // println!("{}", fs);
        });

        fs
    }

    fn checksum(&self) -> i64 {
        self.layout
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                Block::File(id) => Some(i as i64 * (*id as i64)),
                _ => None,
            })
            .sum()
    }
}

impl fmt::Display for FS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.layout.iter() {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

fn main() {
    let mut fs = FS::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            fs.parse(&line.unwrap());
        }
    }

    // println!("{}", fs);

    let fs_1 = fs.moved();

    // println!("{}", fs);

    let result = fs_1.checksum();
    println!("Result: {}", result);

    let fs_2 = fs.file_moved();

    // println!("{}", fs_2);

    let result = fs_2.checksum();

    println!("Result: {}", result);
}
