use std::{io::stdout, time::Duration, thread};
use crossterm::{execute, terminal::{self, ClearType, Clear}, cursor, style::Print, event::{KeyCode, poll, read, self, KeyEventKind}};
use crate::world::{World, up_alive};

pub struct Cgol {
    frame_delay: Duration,
    world: World,
}

impl Cgol {
    pub fn new(frame_delay: Duration, w: i32, h: i32, fname: String) -> Cgol {
        if fname == "" {
            Cgol{frame_delay, world: World::new_rand(w, h).unwrap()}
        } else {
            Cgol{frame_delay, world: World::from_file(fname).unwrap()}
        }
    }

    pub fn run(&mut self) { 
        // Bind io::stdout() output to variable for convenience
        let mut stdout = stdout();
        let mut flag = false;
            
        // Enter a new terminal screen, clear and hide cursor,
        // print initial world state
        execute!(stdout, 
            terminal::EnterAlternateScreen, 
            terminal::Clear(ClearType::All), 
            cursor::Hide, 
            cursor::MoveTo(0, 0), 
            Print(&self.world), 
            Print("\nPress Esc to exit program\nPress Space to pause program"))
            .unwrap();
    
        thread::sleep(self.frame_delay);
    
        'outer: loop {
            thread::sleep(self.frame_delay);

            // Allow user input while program is running
            match get_key() {
                KeyCode::Esc => break,
                KeyCode::Char(' ') => {
                    let mut key = get_key();
                    while key != KeyCode::Char(' ') {
                        key = get_key();
                        if key == KeyCode::Esc {
                            break 'outer;
                        }
                    }
                    let _ = get_key();
                },
                KeyCode::Left => {
                    if !flag && self.frame_delay.as_millis() < 150 {
                        flag = true;
                        self.frame_delay = Duration::from_millis(self.frame_delay.as_millis() as u64 + 5);
                        let _ = get_key();
                    }
                },
                KeyCode::Right => {
                    if !flag && self.frame_delay.as_millis() >= 10 {
                        flag = true;
                        self.frame_delay = Duration::from_millis(self.frame_delay.as_millis() as u64 - 5);
                        let _ = get_key();
                    }
                },
                _ => {
                    flag = false;
                    let _ = get_key();
                },
            }
            
            let prev_world = self.world.clone();
            self.update(&prev_world).unwrap();
        
            if prev_world.world != self.world.world {
                execute!(stdout, 
                    cursor::MoveTo(0, 0),
                    Print(&self.world),
                    cursor::MoveToNextLine(3),
                    terminal::Clear(ClearType::CurrentLine),
                    Print(format!("Current frame delay: {} ms", self.frame_delay.as_millis()))).unwrap();
            } else {
                execute!(stdout, 
                    cursor::MoveTo(0, (self.world.height() + 2) as u16), 
                    Clear(ClearType::CurrentLine), 
                    Print("Press any key to exit program..."))
                    .unwrap();
                read().unwrap();
                break;
            }
        }

        // For debugging
        // thread::sleep(Duration::from_secs(2));
        
        // Make sure to return to original terminal screen
        execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    }

    fn update(&mut self, prev_world: &World) -> Result<(), String> {  
        if self.world.width() != prev_world.width() || self.world.height() != prev_world.height() {
            return Err(String::from("Error comparing current world state with previous world state."));
        }

        for i in 0..self.world.height() as usize {
            for j in 0..self.world.width() as usize {
                self.world.world[i][j] = up_alive(prev_world.world[i][j], prev_world.cell_neighbors_sol(i as i32, j as i32));
            }
        }

        Ok(())
    }
}

fn get_key() -> KeyCode {
    // Look for events. If none, don't block
    if poll(Duration::from_millis(1)).unwrap() {
        // Match on event type
        match read().unwrap() {
            event::Event::Key(k) => {
                if k.kind == KeyEventKind::Press && k.kind != KeyEventKind::Release {
                    k.code
                } else {
                    KeyCode::Null
                }
            },
            _ => KeyCode::Null
        }   
    } else {
        KeyCode::Null
    }
}