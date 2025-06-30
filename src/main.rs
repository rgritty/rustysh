use std::io::{self, Write};
use std::process::Command;
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline("rustysh> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("Failed to add history");
                io::stdout().flush().expect("Failed to flush.");

                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                else if trimmed == "exit" {
                    break;
                }
                else if trimmed == "help" {
                    println!("Available commands: help, exit, [external commands]");
                }

                let mut parts = trimmed.split_whitespace();
                if let Some(cmd) = parts.next() {
                    if cmd == "cd" {
                        let new_dir = parts.next().unwrap_or("/");
                        if let Err(e) = std::env::set_current_dir(new_dir) {
                            eprintln!("cd failed: {e}");
                        }
                        continue;
                    }
                    let args: Vec<&str> = parts.collect();
                    
                    match Command::new(cmd).args(&args).spawn() {
                        Ok(mut child) => {
                            let _ = child.wait();
                        }
                        Err(e) => {
                            eprintln!("Failed to execute '{cmd}': {e}");
                        }
                    }
                }
            }
            Err(_) => break,
        }
        
    }
}
