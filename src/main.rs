use core::time::Duration;
use std::{io::{Write, stdout}, thread};
use crossterm::{execute, queue, cursor::{MoveTo, self}, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, self}, style::{Print, self}, event::{poll, read, KeyCode, Event}};

use game_of_life::startup::{World, Cell};

fn main() {
    /*
        TODO: Allow user to input custom world dimensions
    */

    // Generate new GOL world
    let world = World::new(10, 10);

    // Enter a new terminal screen for convenience
    execute!(stdout(), EnterAlternateScreen).unwrap();

    {
        let mut stdout = stdout();

        // Ensure new screen is cleared and cursor is hidden for aesthetics
        execute!(stdout, Clear(ClearType::All)).unwrap();
        execute!(stdout, cursor::Hide).unwrap();

        loop {
            // Allow program to quit if user presses ESC key
            if poll(Duration::from_millis(50)).unwrap() {
                match read().unwrap() {
                    Event::Key(k) => {
                        match k.code {
                            KeyCode::Esc => break,
                            _ => continue,
                        }
                    }
                    _ => continue,
                }
            }

            let prev_world = world.clone();

            // update_gol(&mut world);

            //if prev_world != world {
                queue!(stdout, Clear(ClearType::All)).unwrap();
                queue!(stdout, MoveTo(0, 0)).unwrap();
                queue!(stdout, Print(&world)).unwrap();
                stdout.flush().unwrap();
            //}

            // Pause execution so that screen doesn't flash too much
            // (Will be superseded by only updating screen if there
            // is a change in the world)
            thread::sleep(Duration::from_millis(50));
        }
    }

    // Make sure to return to original terminal screen
    execute!(stdout(), LeaveAlternateScreen).unwrap();
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