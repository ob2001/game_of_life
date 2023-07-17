use std::{io::stdout, time::Duration};
use crossterm::{execute, terminal::{self, ClearType, Clear}, cursor, style::Print, event::{KeyCode, poll, read, self}};
use std::thread;

use crate::world::World;

pub struct Cgol {
    frame_delay: Duration,
    world: World,
}

impl Cgol {
    pub fn new(frame_delay: Duration, w: i32, h: i32) -> Cgol {
        Cgol{frame_delay, world: World::new_rand(w, h).unwrap()}
    }

    pub fn run(&mut self) { 
    // Bind io::stdout() output to variable for convenience
    let mut stdout = stdout();
        
    // Enter a new terminal screen, clear and hide cursor,
    // and print initial world state
    execute!(stdout, 
        terminal::EnterAlternateScreen, 
        terminal::Clear(ClearType::All), 
        cursor::Hide, 
        cursor::MoveTo(0, 0), 
        Print(&world), 
        Print("\nPress Esc to exit program"))
        .unwrap();

    thread::sleep(frame_delay);

    loop {
        match get_key() {
            // Allow program to quit if user presses ESC key
            KeyCode::Esc => break,
            _ => (),
        }

        let prev_world = world.clone();

        update_gol(&mut world);

        if prev_world != world {
            execute!(stdout, cursor::MoveTo(0, 0), Print(&world)).unwrap();
        } else {
            execute!(stdout, 
                cursor::MoveTo(0, (world.height() + 2) as u16), 
                Clear(ClearType::CurrentLine), 
                Print("Press any key to exit program..."))
                .unwrap();
            read().unwrap();
            break;
        }

        thread::sleep(frame_delay);
    }

    // Make sure to return to original terminal screen
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}

fn get_key() -> KeyCode {
    // Look for events. If none, don't block
    if poll(Duration::from_millis(5)).unwrap() {
        // Match on event type
        match read().unwrap() {
            event::Event::Key(k) => k.code,
            _ => KeyCode::Null,
        }
    } else {
        KeyCode::Null
    }
}

fn update_gol(world: &mut World) {
    for i in 0..world.height() as usize {
        for j in 0..world.width() as usize {
            let neighbors = world.cell_neighbors_sol(i as i32, j as i32);
            world.world[i][j].up_alive(neighbors);
        }
    }

    for i in 0..world.height() as usize {
        for j in 0..world.width() as usize {
            if world.world[i][j].changing {
                world.world[i][j].alive = !world.world[i][j].alive;
            }
            world.world[i][j].changing = false;
        }
    }
}