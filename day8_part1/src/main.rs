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
    let valid_points = process_map(&map, width.try_into().unwrap(), height);
    println!("Valid points: {}", valid_points);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_map(map: &Map, width: i32, height: i32) -> usize {
    // Build a dictionary of unique frequencies and locations
    let antennas = get_antennas(&map, width, height);
    println!("Antennas: {:?}", antennas);

    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for frequency in antennas {
        let antinodes = process_frequency(frequency.1);

        println!("Antinodes for frequency '{}': {:?}", frequency.0, antinodes);

        for antinode in antinodes {
            if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                unique_antinodes.insert(antinode);
            }
        }
    }

    return unique_antinodes.len();
}

fn get_antennas(map: &Map, width: i32, height: i32) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for row in 0..height {
        for col in 0..width {
            let cell = map.cell_at(row.try_into().unwrap(), col.try_into().unwrap());

            if cell.data == '.' {
                continue;
            }

            if antennas.contains_key(&cell.data) {
                let mut locations = antennas.get_mut(&cell.data).unwrap();
                locations.push((row, col));
            }
            else {
                let locations = vec![(row, col)];
                antennas.insert(cell.data, locations);
            }
        }
    }

    return antennas;
}

fn process_frequency(antennas: Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (idx, a) in antennas[0..antennas.len() - 1].iter().enumerate() {
        for b in antennas[idx + 1..antennas.len()].iter() {
            let result = get_antinodes(a, b);

            antinodes.insert(result.0);
            antinodes.insert(result.1);
        }
    }
    return antinodes;
}

fn get_antinodes(a: &(i32, i32), b: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    //println!("Getting antinodes for [{},{}] and [{},{}]...", a.0, a.1, b.0, b.1);

    // Get the differences
    let row_diff = a.0 - b.0;
    let col_diff = a.1 - b.1;

    // Return the antinodes
    return (
        (a.0 + row_diff, a.1 + col_diff),
        (b.0 - row_diff, b.1 - col_diff)
    );
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