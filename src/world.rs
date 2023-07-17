use std::{fmt, cmp::{min, max}};
use rand::Rng;
// use std::fs;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub changing: bool,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Cell{alive, changing: false}
    }

    pub fn up_alive(&mut self, neighbors: i32) {
        let (surv, born) = ([2, 3], [3]);
        self.alive = (self.alive && surv.contains(&neighbors)) || (!self.alive && born.contains(&neighbors));
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alive { write!(f, "O") } 
        else { write!(f, " ") }
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
    pub fn new_empty(w: i32, h: i32) -> Result<World, String> {
        if w <= 0 || h <= 0 {
            return Err(String::from("Invalid world dimensions"));
        }

        let world_gen = (0..w).map(|_| (0..h).map(|_| Cell::new(false)).collect()).collect();
        let u_l = (0..w + 2).map(|_| "-").collect::<String>();
        
        Ok(World{width: w, height: h, world: world_gen, upper_lower: u_l})
    }

    pub fn new_rand(w: i32, h: i32) -> Result<World, String> {
        if w <= 0 || h <= 0 {
            return Err(String::from("Invalid world dimensions"));
        }

        let mut world_gen = Vec::new();
        let mut rng = rand::thread_rng();
        let u_l = (0..w + 2).map(|_| "-").collect::<String>();
        
        for _ in 0..h {
            let mut row_gen = Vec::new();
            for _ in 0..w {
                row_gen.push(Cell::new(rng.gen_range(0..100) < 15));
            }
            world_gen.push(row_gen);
        }
        
        Ok(World{width: w, height: h, world: world_gen, upper_lower: u_l})
    }

    pub fn from_file(_fname: &str) -> Result<World, String> {
        todo!("To implement: import world from .txt file");
    }

    pub fn to_file(_fname: &str) -> Result<(), String> {
        todo!("To implement: export world to .txt file");
    }

    pub fn width(&self) -> i32 {self.width}
    pub fn height(&self) -> i32 {self.height}

    pub fn cell_neighbors_sol(&self, x: i32, y: i32) -> i32 {
        let mut ans = 0;
        for i in max(x - 1, 0)..min(x + 2, self.height) {
            for j in max(y - 1, 0)..min(y + 2, self.width) {
                if (i, j) != (x, y) && self.world[i as usize][j as usize].alive{
                    ans += 1;
                }
            }
        }
        ans
    }

    /*
        TODO: Come back to implement periodic boundaries
     */
    // pub fn cell_neighbors_per(&self, x: i32, y: i32) -> i32 {
    //     let mut ans = 0;
    //     for i in max(x - 1, 0)..min(x + 1, self.height) {
    //         for j in max(y - 1, 0)..min(y + 1, self.width) {
    //             if (i, j) != (x, y) && self.world[i as usize][j as usize].alive{
    //                 ans += 1;
    //             }
    //         }
    //     }
    //     ans
    // }
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