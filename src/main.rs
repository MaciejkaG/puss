use std::env;
use std::fs;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn clear() {
    std::process::Command::new("clear").status().unwrap();
}

fn print_content(scroll_x: u16, scroll_y: u16, content: &str) {
    // Clear the terminal
    clear();
    // Get the terminal size
    let (size_x, size_y) = termion::terminal_size().unwrap();

    let column_range = scroll_x..scroll_x + size_x;
    let row_range = scroll_y..scroll_y + size_y;

    let extracted: Vec<String> = content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if row_range.contains(&(i as u16)) {
                // Adjust column range if it exceeds line length
                let column_range_usize = column_range.start as usize..std::cmp::min(column_range.end as usize, line.len());
                // Return the sliced part of the line, or empty if out of range
                Some(line.get(column_range_usize).unwrap_or("").to_string())
            } else {
                None
            }
        })
        .collect();

    let result = extracted.join("\n");
    print!("{}", result);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let path = "/Users/maciej/Documents/demo/index.js"; // Debug purposes

    clear();

    // Read the file
    println!("Reading your file, please wait...");

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let mut scroll_x = 0;
    let mut scroll_y = 0;

    print_content(scroll_x, scroll_y, &contents);

    // Enable raw mode to capture key presses directly
    enable_raw_mode()?;

    loop {
        // Poll for an event
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        scroll_y -= 1;
                        print_content(scroll_x, scroll_y, &contents);
                    },
                    KeyCode::Down => {
                        scroll_y += 1;
                        print_content(scroll_x, scroll_y, &contents);
                    }
                    KeyCode::Left => {
                        scroll_x -= 1;
                        print_content(scroll_x, scroll_y, &contents);
                    },
                    KeyCode::Right => {
                        scroll_x += 1;
                        print_content(scroll_x, scroll_y, &contents);
                    },
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal settings
    disable_raw_mode()?;
    Ok(())
}
