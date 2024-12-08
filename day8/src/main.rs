use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::Range,
    usize,
};

use itertools::Itertools;

type MapBounds = (Range<usize>, Range<usize>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapElement {
    Antenna(char),
    Empty,
    AntiNode,
}

impl fmt::Display for MapElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapElement::Antenna(c) => write!(f, "{}", c),
            MapElement::Empty => write!(f, "."),
            MapElement::AntiNode => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionedMapElement {
    element: MapElement,
    position: (usize, usize),
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<MapElement>>,
}

impl Map {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add_row(&mut self, to_parse: &str) {
        self.data.push(
            to_parse
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(MapElement::Empty),
                    c if c.is_alphanumeric() => Some(MapElement::Antenna(c)),
                    _ => None,
                })
                .collect(),
        );
    }

    fn get_bounds(&self) -> MapBounds {
        let rows = self.data.len();
        let cols = self.data.iter().map(|r| r.len()).max().unwrap_or(0);

        (0..rows, 0..cols)
    }

    fn group(&self) -> MapGrouped {
        let (xr, yr) = self.get_bounds();
        let mut signals = HashMap::new();

        for y in yr {
            for x in xr.clone() {
                if let MapElement::Antenna(c) = self.data[y][x] {
                    signals
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(PositionedMapElement {
                            element: MapElement::Antenna(c),
                            position: (x, y),
                        });
                }
            }
        }
        MapGrouped {
            map: self,
            bounds: self.get_bounds(),
            signals,
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for element in row {
                write!(f, "{}", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct MapGrouped<'a> {
    map: &'a Map,
    bounds: MapBounds,
    signals: HashMap<char, Vec<PositionedMapElement>>,
}

impl<'a> fmt::Display for MapGrouped<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Map:")?;
        writeln!(f, "====")?;
        writeln!(f, "{}", self.map)?;
        writeln!(f, "Signals:")?;
        writeln!(f, "========")?;
        for (c, p) in &self.signals {
            writeln!(f, "{}: {:?}", c, p)?;
        }
        Ok(())
    }
}

impl<'a> MapGrouped<'a> {
    fn get_dxdy_for<O: IntoIterator<Item = (usize, usize)>>(
        position: (usize, usize),
        others: O,
    ) -> impl Iterator<Item = (i32, i32)> {
        let (x, y) = position;
        let x = x as i32;
        let y = y as i32;

        others.into_iter().map(move |(ox, oy)| {
            let ox = ox as i32;
            let oy = oy as i32;
            (ox - x, oy - y)
        })
    }

    fn get_antinodes(&self, harmonics: bool) -> HashSet<PositionedMapElement> {
        return self
            .signals
            .iter()
            .map(|(_, v)| {
                v.iter().map(|a| {
                    let dxdy = Self::get_dxdy_for(a.position, v.iter().map(|a| a.position));
                    dxdy.filter(|(dx, dy)| *dx != 0 && *dy != 0 || harmonics)
                        .cartesian_product(match harmonics {
                            false => 0..1,
                            true => 0..(self.bounds.0.end.max(self.bounds.1.end + 1) as i32),
                        })
                        .filter_map(|((dx, dy), h)| {
                            let mut x = a.position.0 as i32;
                            let mut y = a.position.1 as i32;
                            // println!("dx: {}, dy: {}, x: {}, y: {}, h: {}", dx, dy, x, y, h);
                            x -= dx * (h + 1);
                            y -= dy * (h + 1);
                            if x.is_negative()
                                || y.is_negative()
                                || !self.bounds.0.contains(&(x as usize))
                                || !self.bounds.1.contains(&(y as usize))
                            {
                                return None;
                            }
                            Some(PositionedMapElement {
                                element: MapElement::AntiNode,
                                position: (x as usize, y as usize),
                            })
                        })
                })
            })
            .flatten()
            .flatten()
            .fold(HashSet::new(), |mut acc, v| {
                acc.insert(v);
                acc
            });
    }
}

fn main() {
    let mut map = Map::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            map.add_row(&line.unwrap());
        }
    }

    let grouped = map.group();

    // println!("{}", grouped);

    let antinodes = grouped.get_antinodes(false);
    // println!("Antinodes: {:?}", antinodes);

    println!("Result: {}", antinodes.len());

    let antinodes = grouped.get_antinodes(true);

    println!("Result (harmonics): {}", antinodes.len());
}
