use std::{fmt::{self, Display, Formatter}, cmp::{min, max}};
use rand::Rng;
// use std::fs;

#[derive(Clone)]
pub struct World {
    width: i32,
    height: i32,
    upper_lower: String,
    pub world: Vec<Vec<bool>>,
}

impl World {
    pub fn new_empty(width: i32, height: i32) -> Result<World, String> {
        if width <= 0 || height <= 0 {
            return Err(String::from("Invalid world dimensions"));
        }
        
        Ok(World { width, height, 
            world: vec![vec![false; width as usize]; height as usize], 
            upper_lower: vec!["\u{2500}"; width as usize].into_iter().collect()
        })
    }

    pub fn new_rand(w: i32, h: i32) -> Result<World, String> {
        let mut world = World::new_empty(w, h)?;
        let mut rng = rand::thread_rng();
        
        for row in &mut world.world {
            for cell in row {
                *cell = rng.gen_range(0..100) < 15;
            }
        }
        
        Ok(world)
    }

    pub fn from_file(fname: String) -> Result<World, String> {
        todo!("To implement: import world from .txt file");
    }

    // pub fn to_file(_fname: &str) -> Result<(), String> {
    //     todo!("To implement: export world to .txt file");
    // }

    pub fn width(&self) -> i32 {self.width}
    pub fn height(&self) -> i32 {self.height}

    pub fn cell_neighbors_sol(&self, x: i32, y: i32) -> i32 {
        let mut ans = 0;
        for i in max(x - 1, 0)..min(x + 2, self.height) {
            for j in max(y - 1, 0)..min(y + 2, self.width) {
                if (i, j) != (x, y) && self.world[i as usize][j as usize] {
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

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\u{250c}{}\u{2510}\n", &self.upper_lower)?;
        for row in self.world.as_slice() {
            write!(f, "\u{2502}")?;
            for item in row {
                write!(f, "{}", match item { true => "O", false => " " })?;
            }
            write!(f, "\u{2502}\n")?;
        }
        write!(f, "\u{2514}{}\u{2518}", &self.upper_lower)
    }
}

pub fn up_alive(alive: bool, neighbors: i32) -> bool {
    let (surv, born) = ([2, 3], [3]);
    alive && surv.contains(&neighbors) || !alive && born.contains(&neighbors)
}