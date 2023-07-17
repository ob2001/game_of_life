use std::{fmt, cmp::{min, max}};
use rand::Rng;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub changing: bool,
    pub x: i32,
    pub y: i32,
}

impl Cell {
    fn new(alive: bool, x: i32, y: i32) -> Cell {
        Cell{alive, changing: false, x, y}
    }

    pub fn up_alive(&mut self, neighbors: i32) {
        if self.alive && (neighbors < 2 || neighbors > 3) {
            self.changing = true;
        } else if self.alive && (neighbors == 2 || neighbors == 3) {
            self.changing = false;
        } else if !self.alive && neighbors == 3 {
            self.changing = true;
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alive {
            write!(f, "X")
        } else {
            write!(f, " ")
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        !(self.alive ^ other.alive)
    }
}

#[derive(Clone)]
pub struct World {
    width: i32,
    height: i32,
    upper_lower: String,
    pub world: Vec<Vec<Cell>>,
}

impl World {
    pub fn new_empty(w: i32, h: i32) -> World {
        let world_gen = (0..h).map(|x| (0..w).map(|y| Cell::new(false, x, y)).collect()).collect();
        let u_l = (0..w + 2).map(|_| "-").collect::<String>();
        
        World{width: w, height: h, world: world_gen, upper_lower: u_l}
    }
    pub fn new_rand(w: i32, h: i32) -> World {
        let mut world_gen = Vec::new();
        let mut rng = rand::thread_rng();
        let u_l = (0..w + 2).map(|_| "-").collect::<String>();

        for i in 0..w {
            let mut row_gen = Vec::new();
            for j in 0..h {
                row_gen.push(Cell::new(rng.gen_range(0..100) < 15, i, j));
            }
            world_gen.push(row_gen);
        }

        World{width: w, height: h, world: world_gen, upper_lower: u_l}
    }

    pub fn width(&self) -> i32 {self.width}
    pub fn height(&self) -> i32 {self.height}

    pub fn from_file() -> World {
        todo!("To implement: import world from .txt file");
    }

    pub fn cell_neighbors_sol(&self, i: i32, j: i32) -> i32 {
        let mut ans = 0;
        for i in max(i - 1, 0)..min(i + 1, self.width) {
            for j in max(j - 1, 0)..min(j + 1, self.height) {
                if self.world[i as usize][j as usize].alive{
                    ans += 1;
                }
            }
        }
        ans
    }

    pub fn cell_neighbors_per(&self, x: i32, y: i32) -> i32 {
        let mut ans = 0;
        for i in max(x - 1, 0)..min(x + 1, self.width) {
            for j in max(y - 1, 0)..min(y + 1, self.height) {
                if self.world[i as usize][j as usize].alive{
                    ans += 1;
                }
            }
        }
        ans
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::from("");
        out.push_str(&self.upper_lower);
        out.push('\n');
        for row in self.world.as_slice() {
            out.push('|');
            for item in row {
                out.push_str(item.to_string().as_str());
            }
            out.push_str("|\n");
        }
        out.push_str(&self.upper_lower);
        write!(f, "{}", out)
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.world == other.world
    }
}