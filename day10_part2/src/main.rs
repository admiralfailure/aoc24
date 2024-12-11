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
    let score = process_map(&map, width, height);
    println!("Score: {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_map(map: &Map, width: usize, height: usize) -> usize {
    let mut score = 0;

    // Loop the map looking for 0s
    for row in 0..height {
        for col in 0..width {
            let cell = map.cell_at(row, col);
            if cell.data != '0' {
                continue;
            }

            // For each 0, try to traverse the map to get to a 9
            // If we get to one, increment the counter
            println!("Found trailhead at [{},{}]", cell.row(), cell.col());
            let trailhead_score = process_cell(&map, &cell);
            score += trailhead_score;
            println!("Trailhead score: {}", trailhead_score);
            println!("New total score: {}", score);
        }
    }

    return score;
}

fn process_cell(map: &Map, cell: &Cell) -> usize {
    let mut score = 0;

    let cell_value = cell.data.to_digit(10).unwrap();
    for neighbour in cell.neighbours(&Direction::Square).all() {
        // Is this a valid route (i.e. exactly one greater than this cell)?
        let neighbour_value = neighbour.data.to_digit(10).unwrap();
        if neighbour_value == cell_value + 1 {
            // Is the neighbour a 9?
            if neighbour_value == 9 {
                score += 1;
            }
            else {
                // Else, recurse down this path
                score += process_cell(&map, &neighbour);
            }
        }
    }

    return score;
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
        if (*direction == Direction::All || *direction == Direction::TopLeft) && (self.idx > self.map.row_width) && (self.idx > 0 && self.idx % self.map.row_width > 0) {
            neighbours.top_left = Some(Cell::new(&self.map, (self.idx - self.map.row_width) - 1));
        }

        // Top
        if (*direction == Direction::All || *direction == Direction::Square || *direction == Direction::Top) && self.idx > self.map.row_width {
            neighbours.top = Some(Cell::new(&self.map, self.idx - self.map.row_width));
        }

        // Top Right
        if (*direction == Direction::All || *direction == Direction::TopRight) && (self.idx > self.map.row_width) && (self.idx % self.map.row_width < self.map.row_width - 1) {
            neighbours.top_right = Some(Cell::new(&self.map, (self.idx - self.map.row_width) + 1));
        }

        // Left
        if (*direction == Direction::All || *direction == Direction::Square || *direction == Direction::Left) && self.idx > 0 && self.idx % self.map.row_width > 0 {
            neighbours.left = Some(Cell::new(&self.map, self.idx - 1));
        }

        // Right
        if (*direction == Direction::All || *direction == Direction::Square || *direction == Direction::Right) && self.idx % self.map.row_width < self.map.row_width - 1 {
            neighbours.right = Some(Cell::new(&self.map, self.idx + 1));
        }

        // Bottom Left
        if (*direction == Direction::All || *direction == Direction::BottomLeft) && (self.idx + self.map.row_width < self.map.data.len()) && (self.idx > 0 && self.idx % self.map.row_width > 0) {
            neighbours.bottom_left = Some(Cell::new(&self.map, (self.idx + self.map.row_width) - 1));
        }

        // Bottom
        if (*direction == Direction::All || *direction == Direction::Square || *direction == Direction::Bottom) && self.idx + self.map.row_width < self.map.data.len() {
            neighbours.bottom = Some(Cell::new(&self.map, self.idx + self.map.row_width));
        }

        // Bottom Right
        if (*direction == Direction::All || *direction == Direction::BottomRight) && (self.idx + self.map.row_width < self.map.data.len()) && (self.idx % self.map.row_width < self.map.row_width - 1) {
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

    Square,

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

    fn all(&self) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::new();

        if let Some(top_left) = &self.top_left {
            cells.push(top_left);
        }
        if let Some(top) = &self.top {
            cells.push(top);
        }
        if let Some(top_right) = &self.top_right {
            cells.push(top_right);
        }
        if let Some(left) = &self.left {
            cells.push(left);
        }
        if let Some(right) = &self.right {
            cells.push(right);
        }
        if let Some(bottom_left) = &self.bottom_left {
            cells.push(bottom_left);
        }
        if let Some(bottom) = &self.bottom {
            cells.push(bottom);
        }
        if let Some(bottom_right) = &self.bottom_right {
            cells.push(bottom_right);
        }

        return cells;
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