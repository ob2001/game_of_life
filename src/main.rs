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