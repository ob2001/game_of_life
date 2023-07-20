use game_of_life::startup::Cgol;
use std::{time::Duration, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fname = String::new();
    let (mut frame_delay, mut w, mut h) =  (Duration::from_millis(50), 40, 20);

    
    if args.contains(&String::from("--dims")) {
        let idx = args
            .iter()
            .position(|r| r == "dims")
            .unwrap();
        (w, h) = (args[idx + 1].parse().unwrap(),
            args[idx + 2].parse().unwrap());
    }

    if args.contains(&String::from("--delay")) {
        let idx = args
            .iter()
            .position(|r| r == "delay")
            .unwrap();
        frame_delay = Duration::from_millis(args[idx + 1].parse().unwrap());
    }

    if args.contains(&String::from("-f")) {
        let idx = args
            .iter()
            .position(|r| r == "-f")
            .unwrap();
        fname = args[idx + 1].clone();
    }
    
    // Create and run new GOL world from given arguments (if any)
    Cgol::new(frame_delay, w, h, fname).run();
}