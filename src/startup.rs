use std::{io::{stdout, Write}, time::Duration};
use crossterm::{execute, queue, terminal::{self, ClearType}, cursor, style::Print, event::{KeyCode, poll, read, self}};
// use std::fs;

use crate::world::World;

pub fn run(world: World) {
    let mut prev_world = World::new_empty(world.width, world.height);

    // Bind io::stdout() output to variable for convenience
    let mut stdout = stdout();
        
    // Enter a new terminal screen, clear and hide cursor,
    // and print initial world state
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0), Print(&world)).unwrap();

    loop {
        match get_key().unwrap() {
            // Allow program to quit if user presses ESC key
            KeyCode::Esc => break,
            _ => (),
        }

        prev_world = world.clone();

        // update_gol(&mut world);

        if prev_world != world {
            queue!(stdout, terminal::Clear(ClearType::All)).unwrap();
            queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
            queue!(stdout, Print(&world)).unwrap();
            stdout.flush().unwrap();
        }
    }

    // Make sure to return to original terminal screen
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}

fn get_key() -> Result<KeyCode, ()> {
    // Look for events. If none, don't block
    if poll(Duration::from_millis(5)).unwrap() {
        // Match on event type
        match read().unwrap() {
            event::Event::Key(k) => Ok(k.code),
            _ => Ok(KeyCode::Null),
        }
    } else {
        Ok(KeyCode::Null)
    }
}