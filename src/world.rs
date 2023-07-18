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
        write!(f, "{}", match self.alive { true => "O", false => " "})
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool { !(self.alive ^ other.alive) }
}

#[derive(Clone)]
pub struct World {
    width: i32,
    height: i32,
    upper_lower: String,
    pub world: Vec<Vec<Cell>>,
}

impl World {
    pub fn new_empty(width: i32, height: i32) -> Result<World, String> {
        if width <= 0 || height <= 0 {
            return Err(String::from("Invalid world dimensions"));
        }

        Ok(World { width, height, 
            world: vec![vec![Cell::new(false); width as usize]; height as usize], 
            upper_lower: vec!["-"; width as usize + 2].into_iter().collect()
        })
    }

    pub fn new_rand(w: i32, h: i32) -> Result<World, String> {
        let mut world = World::new_empty(w, h)?;
        let mut rng = rand::thread_rng();
        
        for row in &mut world.world {
            for cell in row {
                cell.alive = rng.gen_range(0..100) < 15;
            }
        }
        
        Ok(world)
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
        write!(f, "{}\n", &self.upper_lower)?;
        for row in self.world.as_slice() {
            write!(f, "|")?;
            for item in row {
                write!(f, "{}", item)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "{}", &self.upper_lower)
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.world == other.world
    }
}