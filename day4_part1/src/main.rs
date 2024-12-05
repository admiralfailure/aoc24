use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

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

    let mut total = 0;
    for row in 0..height {
        for col in 0..width {
            let cell = map.cell_at(row, col);

            if cell.data != 'X' {
                continue;
            }

            println!("Found {} at [{}, {}]", cell.data, row, col);
            
            let _ms = check_cell_neighbours(&cell,'M', Direction::All);
            for m in _ms {


                let _as = check_cell_neighbours(&m.0, 'A', m.1);
                for a in _as {
                    let _ss = check_cell_neighbours(&a.0, 'S', a.1);
                    for s in _ss {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("Total: {}", total);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn check_cell_neighbours<'a>(cell: &'a Cell<'a>, target: char, direction: Direction) -> Vec<(Cell<'a>, Direction)> {
    let mut result: Vec<(Cell<'a>, Direction)> = Vec::new();

    let neighbours = cell.neighbours(direction);

    match neighbours.top_left {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::TopLeft))
            }
        },
        None => {}
    }
    match neighbours.top {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::Top))
            }
        },
        None => {}
    }
    match neighbours.top_right {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::TopRight))
            }
        },
        None => {}
    }
    match neighbours.left {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::Left))
            }
        },
        None => {}
    }
    match neighbours.right {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::Right))
            }
        },
        None => {}
    }
    match neighbours.bottom_left {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::BottomLeft))
            }
        },
        None => {}
    }
    match neighbours.bottom {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::Bottom))
            }
        },
        None => {}
    }
    match neighbours.bottom_right {
        Some(x) => {
            if x.data == target {
                result.push((x, Direction::BottomRight))
            }
        },
        None => {}
    }
    
    return result;
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

    pub fn cell_at(&self, row: usize, col: usize) -> Cell {
        let idx = (row * self.row_width) + col;
        return Cell::new(self, idx);
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

    fn neighbours(&self, direction: Direction) -> Neighbours {
        let mut neighbours = Neighbours::new();

        // Top Left
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::TopLeft)) && (self.idx > self.map.row_width) && (self.idx > 0 && self.idx % self.map.row_width > 0)) {
            neighbours.top_left = Some(Cell::new(&self.map, (self.idx - self.map.row_width) - 1));
        }

        // Top
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::Top)) && self.idx > self.map.row_width) {
            neighbours.top = Some(Cell::new(&self.map, self.idx - self.map.row_width));
        }

        // Top Right
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::TopRight)) &&(self.idx > self.map.row_width) && (self.idx % self.map.row_width < self.map.row_width - 1)) {
            neighbours.top_right = Some(Cell::new(&self.map, (self.idx - self.map.row_width) + 1));
        }

        // Left
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::Left)) &&self.idx > 0 && self.idx % self.map.row_width > 0) {
            neighbours.left = Some(Cell::new(&self.map, self.idx - 1));
        }

        // Right
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::Right)) &&self.idx % self.map.row_width < self.map.row_width - 1) {
            neighbours.right = Some(Cell::new(&self.map, self.idx + 1));
        }

        // Bottom Left
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::BottomLeft)) &&(self.idx + self.map.row_width < self.map.data.len()) && (self.idx > 0 && self.idx % self.map.row_width > 0)) {
            neighbours.bottom_left = Some(Cell::new(&self.map, (self.idx + self.map.row_width) - 1));
        }

        // Bottom
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::Bottom)) &&self.idx + self.map.row_width < self.map.data.len()) {
            neighbours.bottom = Some(Cell::new(&self.map, self.idx + self.map.row_width));
        }

        // Bottom Right
        if ((matches!(direction, Direction::All) || matches!(direction, Direction::BottomRight)) &&(self.idx + self.map.row_width < self.map.data.len()) && (self.idx % self.map.row_width < self.map.row_width - 1)) {
            neighbours.bottom_right = Some(Cell::new(&self.map, (self.idx + self.map.row_width) + 1));
        }

        return neighbours;
    }
}

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
}