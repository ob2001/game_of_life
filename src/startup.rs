use std::{io::stdout, time::Duration};
use crossterm::{execute, terminal::{self, ClearType, Clear}, cursor, style::Print, event::{KeyCode, poll, read, self}};
use std::thread;

use crate::world::{World, up_alive};

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
            Print(&self.world), 
            Print("\nPress Esc to exit program"))
            .unwrap();
    
        thread::sleep(self.frame_delay);
    
        loop {
            match get_key() {
                // Allow program to quit if user presses ESC key
                KeyCode::Esc => break,
                KeyCode::Char(' ') => (),
                _ => (),
            }
    
            let prev_world = self.world.clone();
    
            self.world = self.update();
    
            if prev_world != self.world {
                execute!(stdout, cursor::MoveTo(0, 0), Print(&self.world)).unwrap();
            } else {
                execute!(stdout, 
                    cursor::MoveTo(0, (self.world.height() + 2) as u16), 
                    Clear(ClearType::CurrentLine), 
                    Print("Press any key to exit program..."))
                    .unwrap();
                read().unwrap();
                break;
            }
    
            thread::sleep(self.frame_delay);
        }
    
        // For debugging
        // thread::sleep(Duration::from_secs(2));

        // Make sure to return to original terminal screen
        execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    }

    fn update(&self) -> World {
        let mut temp = self.world.clone();
        
        for i in 0..temp.height() {
            for j in 0..temp.width() {
                temp.world[i as usize][j as usize] = up_alive(temp.world[i as usize][j as usize], self.world.cell_neighbors_sol(i, j));
            }
        }

        temp
    }
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
