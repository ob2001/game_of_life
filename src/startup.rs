use std::fmt;
use rand::Rng;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Cell{alive}
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
    pub fn new(w: i32, h: i32) -> World {
        let mut world_gen = Vec::new();
        let mut rng = rand::thread_rng();
        let u_l = (0..w + 2).map(|_| "-").collect::<String>();

        for _ in 0..w {
            let mut row_gen = Vec::new();
            for _ in 0..h {
                row_gen.push(Cell::new(rng.gen_range(0..100) < 10));
            }
            world_gen.push(row_gen);
        }

        World{width: w, height: h, world: world_gen, upper_lower: u_l}
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