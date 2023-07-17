use game_of_life::{
    world::{World, Cell}, 
    startup::run};

fn main() {
    /*
        TODO: Allow user to input custom world dimensions
    */

    // Generate and run new GOL world
    run(World::new_rand(25, 25));
}

fn update_gol(world: &mut World) {
    for i in 0..world.world.len() {
        for j in 0..world.world[0].len() {
            up_life(&mut world.world[i][j], vec![]);
        }
    }
}

fn up_life(cell: &mut Cell, surr: Vec<&mut Cell>) -> Cell {
    todo!("To implement: ");
}