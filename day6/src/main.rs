use std::{collections::HashSet, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MapElement {
    None,
    Obstacle,
    Guard(Direction),
}

impl MapElement {
    fn is_guard(&self) -> bool {
        match self {
            MapElement::Guard(_) => true,
            _ => false,
        }
    }

    fn is_obstacle(&self) -> bool {
        match self {
            MapElement::Obstacle => true,
            _ => false,
        }
    }

    fn get_guard_movement_vector(&self) -> (i32, i32) {
        match self {
            MapElement::Guard(Direction::Up) => (0, -1),
            MapElement::Guard(Direction::Down) => (0, 1),
            MapElement::Guard(Direction::Left) => (-1, 0),
            MapElement::Guard(Direction::Right) => (1, 0),
            _ => unreachable!(),
        }
    }

    fn get_guard_direction(&self) -> Direction {
        match self {
            MapElement::Guard(direction) => *direction,
            _ => unreachable!(),
        }
    }

    fn turn_90(&self) -> Self {
        match self {
            MapElement::Guard(direction) => MapElement::Guard(direction.turn_90()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<MapElement>>,
}

impl Map {
    fn new() -> Self {
        Map { map: Vec::new() }
    }

    fn add_line(&mut self, line: &str) {
        let mut row = Vec::new();
        for c in line.chars().filter(char::is_ascii) {
            let element = match c {
                '.' => MapElement::None,
                '#' => MapElement::Obstacle,
                '^' => MapElement::Guard(Direction::Up),
                'v' => MapElement::Guard(Direction::Down),
                '<' => MapElement::Guard(Direction::Left),
                '>' => MapElement::Guard(Direction::Right),
                _ => unreachable!(),
            };
            row.push(element);
        }
        self.map.push(row);
    }

    fn simulate_internal(&mut self, use_history: bool) -> (i32, HashSet<(i32, i32)>) {
        // Define map bounds
        let horizontal = 0..(self.map[0].len() as i32);
        let vertical = 0..(self.map.len() as i32);
        let is_in_bounds =
            |x: i32, y: i32| -> bool { horizontal.contains(&x) && vertical.contains(&y) };

        let guard = self.find_guard();

        // Remove guard from map
        self.map[guard.1 as usize][guard.0 as usize] = MapElement::None;

        let (mut x, mut y, mut guard) = guard;

        let mut history = HashSet::new();
        let mut visited = HashSet::new();
        visited.insert((x, y));

        enum StepResult {
            OutOfBounds,
            StuckInLoop,
            None,
        }
        let mut step = || -> StepResult {
            visited.insert((x, y));
            let guard_historic_record = (x, y, guard.get_guard_direction());
            if history.contains(&guard_historic_record) && use_history {
                return StepResult::StuckInLoop;
            }
            history.insert(guard_historic_record);
            let (dx, dy) = guard.get_guard_movement_vector();
            (x, y) = (x + dx, y + dy);
            if !is_in_bounds(x + dx, y + dy) {
                visited.insert((x, y));
                return StepResult::OutOfBounds;
            }
            let facing = self.map[(y + dy) as usize][(x + dx) as usize];
            if facing.is_obstacle() {
                guard = guard.turn_90();
            }
            StepResult::None
        };

        loop {
            match step() {
                StepResult::OutOfBounds => break,
                StepResult::StuckInLoop => return (-1, visited),
                StepResult::None => (),
            }
        }

        (visited.len() as i32, visited)
    }

    fn simulate(&mut self, use_history: bool) -> i32 {
        let result = self.simulate_internal(use_history);
        result.0
    }

    fn find_guard(&self) -> (i32, i32, MapElement) {
        // Find guard with cords
        let guard = self
            .map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, element)| match element.is_guard() {
                        true => Some((x as i32, y as i32, element.to_owned())),
                        false => None,
                    })
            })
            .expect("No guard found");
        guard
    }

    fn permutate(&self) -> MapPermutator {
        MapPermutator::new(self)
    }
}

struct MapPermutator<'a> {
    map: &'a Map,
    potential_spots: Vec<(i32, i32)>,
    guard: (i32, i32),
}

impl<'a> MapPermutator<'a> {
    fn new(map: &'a Map) -> Self {
        let mut opt_map = map.clone();
        let (_, potential_spots) = opt_map.simulate_internal(false);
        let (x, y, _) = map.find_guard();
        let potential_spots = potential_spots.into_iter().collect();
        MapPermutator { map, guard: (x, y), potential_spots }
    }
}

impl Iterator for MapPermutator<'_> {
    type Item = Map;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.potential_spots.pop()?;
        if (x, y) == self.guard {
            return self.next();
        }
        let mut map = self.map.clone();
        map.map[y as usize][x as usize] = MapElement::Obstacle;
        Some(map)
    }
    
}

fn main() {
    let mut map = Map::new();
    let stdin = std::io::stdin();
    {
        let lock = stdin.lock();

        for line in lock.lines() {
            map.add_line(&line.unwrap());
        }
    }
    let clean_map = map.clone();
    let result = map.simulate(false);

    println!("Result: {}", result);

    // Part 2
    // Whole code bellow and code paths for it may not be correct
    let result2 = clean_map.permutate().map(|mut map| {
        let simulation_result = map.simulate(true);
        // println!("Simulation result: {}", simulation_result);
        match simulation_result {
            -1 => 1,
            _ => 0,
        }
    }).sum::<i32>();

    println!("Result: {}", result2);
}
