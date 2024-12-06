use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() {
    let input_path = "input.txt";
    let mut content: String = Default::default();

    let mut width = 0;
    let mut height = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                content.push_str(&line_value);

                width = line_value.len();
                height += 1;
            }
        }
    }

    let map = Map::new(&content, width);

    // Find our guard
    let mut guard: (usize, usize) = (0,0);
    for row in 0..height {
        for col in 0..width {
            let cell = map.cell_at(row, col);

            if cell.data == '^' {
                guard = (row, col);
                println!("Found guard at [{}, {}]", row, col);
            }
        }
    }

    // Run the standard map once to get visited cells
    let cell = map.cell_at(guard.0, guard.1);
    let visited = get_visited_cells(&cell);

    // Process a new map
    let mut total = 0;
    for cell in visited {
        let (row, col) = map.idx_to_row_col(cell);

        if row == guard.0 && col == guard.1 {
            continue;
        }

        println!("Modified cell [{},{}]...", row, col);
        let mut modified_map = Map::new(&content, width);
        if modified_map.cell_at(row, col).data != '#' {
            modified_map.set_data(row, col, '#');

            if calculate_guard_moves(&modified_map.cell_at(guard.0, guard.1)) {
                total += 1;
            }
        }
    }
    // let mut total = 0;
    // for row in 0..height {
    //     for col in 0..width {
    //         if row == guard.0 && col == guard.1 {
    //             continue;
    //         }

    //         println!("Modified cell [{},{}]...", row, col);
    //         let mut modified_map = Map::new(&content, width);
    //         if modified_map.cell_at(row, col).data != '#' {
    //             modified_map.set_data(row, col, '#');

    //             if calculate_guard_moves(&modified_map.cell_at(guard.0, guard.1)) {
    //                 total += 1;
    //             }
    //         }
    //     }
    // }

    println!("{} infinite loops", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn get_visited_cells(guard: &Cell) -> Vec<usize> {
    let direction = Direction::Top;
    let mut visited: HashMap<usize, HashSet<&Direction>> = HashMap::new();

    calculate_guard_move(guard, &direction, &mut visited);

    return visited.into_keys().collect();
}

fn calculate_guard_moves(guard: &Cell) -> bool {
    let direction = Direction::Top;
    let mut visited: HashMap<usize, HashSet<&Direction>> = HashMap::new();

    let is_loop = calculate_guard_move(guard, &direction, &mut visited);

    return is_loop;
}

fn calculate_guard_move<'a>(guard: &Cell, direction: &'a Direction, visited: &mut HashMap<usize, HashSet<&'a Direction>>) -> bool {
    //println!("Moving guard at [{},{}]", guard.row(), guard.col());

    // Have we been here before?
    if visited.contains_key(&guard.idx) {
        let visit: &mut HashSet<&Direction> = visited.get_mut(&guard.idx).unwrap();
        if visit.contains(&direction) {
            return true;
        }
    }
    else {
        let mut content: HashSet<&Direction> = HashSet::new();
        content.insert(direction);
        visited.insert(guard.idx, content);
    }

    match guard.neighbours(&direction).get(&direction) {
        Some(x) => {
            if x.data == '.' || x.data == '^' {
                // Free space; recurse
                return calculate_guard_move(x, direction, visited);
            }
            else if x.data == '#' {
                // Turn clockwise from current position
                return calculate_guard_move(guard, &turn_clockwise(direction), visited);
            }
        },
        None => {}
    }

    return false;
}

fn turn_clockwise(direction: &Direction) -> &Direction {
    match direction {
        Direction::Top => return &Direction::Right,
        Direction::Right => return &Direction::Bottom,
        Direction::Bottom => return &Direction::Left,
        Direction::Left => return &Direction::Top,
        _ => return &direction
    }
}

struct Map {
    data: Vec<char>,
    row_width: usize
}

impl Map {
    pub fn new(data: &str, row_width: usize) -> Self {
        return Self {
            data: data.chars().collect(),
            row_width: row_width
        };  
    }

    pub fn idx_to_row_col(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.row_width;
        let col = idx % self.row_width;

        return (row, col);
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Cell {
        let idx = (row * self.row_width) + col;
        return Cell::new(self, idx);
    }

    pub fn set_data(&mut self, row: usize, col: usize, new_data: char) {
        let idx = (row * self.row_width) + col;
        self.data[idx] = new_data;
    }
}

struct Cell<'a> {
    map: &'a Map,
    idx: usize,
    data: char
}

impl<'a> Cell<'a> {
    fn new(map: &'a Map, idx: usize) -> Self {
        return Self {
            map,
            idx,
            data: map.data[idx]
        };
    }

    fn row(&self) -> usize {
        return &self.idx / &self.map.row_width;
    }

    fn col(&self) -> usize {
        return &self.idx % &self.map.row_width;
    }

    fn neighbours(&self, direction: &Direction) -> Neighbours {
        let mut neighbours = Neighbours::new();

        // Top Left
        if (matches!(direction, Direction::All) || matches!(direction, Direction::TopLeft)) && (self.idx > self.map.row_width) && (self.idx > 0 && self.idx % self.map.row_width > 0) {
            neighbours.top_left = Some(Cell::new(&self.map, (self.idx - self.map.row_width) - 1));
        }

        // Top
        if (matches!(direction, Direction::All) || matches!(direction, Direction::Top)) && self.idx > self.map.row_width {
            neighbours.top = Some(Cell::new(&self.map, self.idx - self.map.row_width));
        }

        // Top Right
        if (matches!(direction, Direction::All) || matches!(direction, Direction::TopRight)) &&(self.idx > self.map.row_width) && (self.idx % self.map.row_width < self.map.row_width - 1) {
            neighbours.top_right = Some(Cell::new(&self.map, (self.idx - self.map.row_width) + 1));
        }

        // Left
        if (matches!(direction, Direction::All) || matches!(direction, Direction::Left)) &&self.idx > 0 && self.idx % self.map.row_width > 0 {
            neighbours.left = Some(Cell::new(&self.map, self.idx - 1));
        }

        // Right
        if (matches!(direction, Direction::All) || matches!(direction, Direction::Right)) &&self.idx % self.map.row_width < self.map.row_width - 1 {
            neighbours.right = Some(Cell::new(&self.map, self.idx + 1));
        }

        // Bottom Left
        if (matches!(direction, Direction::All) || matches!(direction, Direction::BottomLeft)) &&(self.idx + self.map.row_width < self.map.data.len()) && (self.idx > 0 && self.idx % self.map.row_width > 0) {
            neighbours.bottom_left = Some(Cell::new(&self.map, (self.idx + self.map.row_width) - 1));
        }

        // Bottom
        if (matches!(direction, Direction::All) || matches!(direction, Direction::Bottom)) &&self.idx + self.map.row_width < self.map.data.len() {
            neighbours.bottom = Some(Cell::new(&self.map, self.idx + self.map.row_width));
        }

        // Bottom Right
        if (matches!(direction, Direction::All) || matches!(direction, Direction::BottomRight)) &&(self.idx + self.map.row_width < self.map.data.len()) && (self.idx % self.map.row_width < self.map.row_width - 1) {
            neighbours.bottom_right = Some(Cell::new(&self.map, (self.idx + self.map.row_width) + 1));
        }

        return neighbours;
    }

    // fn set_data(&mut self, new_data: char) {
    //     self.map.data[self.idx] = new_data;
    //     self.data = self.map.data[self.idx];
    // }
}

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    All,

    TopLeft,
    Top,
    TopRight,

    Left,
    Right,

    BottomLeft,
    Bottom,
    BottomRight
}

struct Neighbours<'a> {
    top_left: Option<Cell<'a>>,
    top: Option<Cell<'a>>,
    top_right: Option<Cell<'a>>,
    left: Option<Cell<'a>>,
    right: Option<Cell<'a>>,
    bottom_left: Option<Cell<'a>>,
    bottom: Option<Cell<'a>>,
    bottom_right: Option<Cell<'a>>,
}

impl<'a> Neighbours<'a> {
    fn new() -> Self {
        return Self {
            top_left: None,
            top: None,
            top_right: None,
            left: None,
            right: None,
            bottom_left: None,
            bottom: None,
            bottom_right: None,
        }
    }

    fn get(&self, direction: &Direction) -> &Option<Cell<'a>> {
        return match direction {
            Direction::Top => &self.top,
            Direction::Right => &self.right,
            Direction::Bottom => &self.bottom,
            Direction::Left => &self.left,
            _ => &None
        }
    }
}