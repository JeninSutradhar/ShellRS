// Code Documentation and Explaination
use ansi_term::Color;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio, Child};

/// Main function that starts the program.
fn main() {
    // Create a new instance of the Editor struct
    let mut rl = Editor::<()>::new();
    
    // Load the history from the history.txt file
    // If there's an error, print a message
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // Loop until the loop is broken
    loop {
        // Read a line of input from the user
        // Add a prompt in green color to the beginning of the line
        let readline = rl.readline(&format!("{} ", Color::Green.bold().paint(">")));

        // Match the result of the readline function
        match readline {
            // If the readline is successful, add the line to the history,
            // and call the handle_command function with the line as an argument
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_command(line);
            }
            // If the readline is interrupted by Ctrl-C, print "CTRL-C" in red color and break the loop
            Err(ReadlineError::Interrupted) => {
                println!("{}", Color::Red.bold().paint("CTRL-C"));
                break;
            }
            // If the readline is interrupted by Ctrl-D, print "CTRL-D" in red color and break the loop
            Err(ReadlineError::Eof) => {
                println!("{}", Color::Red.bold().paint("CTRL-D"));
                break;
            }
            // If there's an error, print the error message in red color and break the loop
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save the history to the history.txt file
    rl.save_history("history.txt").unwrap();
}

/// Handles the command by parsing it and executing it.
///
/// # Arguments
///
/// * `input` - The input command as a string.
fn handle_command(input: String) {
    // Must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    // Loop through each command
    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        
        // Check if parts.next() returns Some value, otherwise continue
        if let Some(command) = parts.next() {
            let args: Vec<&str> = parts.collect();

            // Match the command
            match command {
                // Change directory
                "cd" => {
                    // Get the new directory from the arguments
                    let new_dir = args.get(0).map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    // Try to change the directory
                    if let Err(e) = env::set_current_dir(&root) {
                        // If the directory doesn't exist, print an error
                        eprintln!("{}", Color::Red.paint(format!("No such file or directory: {}", new_dir)));
                    }
                    previous_command = None;
                },
                // Quit the program
                "exit" => std::process::exit(0),
                // Execute the command
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(), |output: Child| {
                            Stdio::from(output.stdout.expect("Failed to get stdout"))
                        });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(&args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(mut child) => {
                            let ecode = child.wait().expect("Failed to wait on child");
                            if !ecode.success() {
                                eprintln!("{}", Color::Red.paint(format!("Command exited with status: {}", ecode)));
                            }
                            previous_command = None;
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", Color::Red.paint(e.to_string()));
                        },
                    };
                }
            }
        } else {
            // If the command is empty, skip the iteration
            continue;
        }
    }
}
