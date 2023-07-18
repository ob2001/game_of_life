use game_of_life::startup::Cgol;
use std::time::Duration;

fn main() {
    /*
        TODO: Allow user to input custom world dimensions, frame rate, [? live cell character]
    */

    // Create and run new GOL world
    Cgol::new(Duration::from_millis(50), 175, 40).run();
}