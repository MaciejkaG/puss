use std::env;
use std::fs;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

fn clear() {
    std::process::Command::new("clear").status().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    clear();
    println!("Reading your file, please wait...");

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    // Enable raw mode to capture key presses directly
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    loop {
        // Poll for an event
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => println!("Up arrow pressed"),
                    KeyCode::Down => println!("Down arrow pressed"),
                    KeyCode::Left => println!("Left arrow pressed"),
                    KeyCode::Right => println!("Right arrow pressed"),
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    clear();
    println!("{}", contents);

    // Restore terminal settings
    disable_raw_mode()?;
    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
