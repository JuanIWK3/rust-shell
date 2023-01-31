use std::env;
use std::io::{self, Write};
use std::process::Command;

fn exec_windows(command: &str) {
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command.trim())
        .output()
        .expect("Failed to run!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn exec_linux(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.trim())
        .output()
        .expect("Failed to run!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let os = env::consts::OS;

    loop {
        let mut command = String::new();

        print!("$ ");

        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line!");

        let info = os_info::get();

        let command = command.trim();

        if command == "exit" {
            break;
        } else if command == "ver" {
            println!("{}", info);
        }

        if command.starts_with("cd ") {
            let dir = &command[3..];
            env::set_current_dir(dir).expect("Failed to change dirs");
        } else if command.starts_with("edit ") {
            let path = &command[5..];

            let mut editor = Command::new("notepad")
                .arg(path)
                .spawn()
                .expect("Failed to open nano editor");

            editor.wait().unwrap();
        } else {
            if os == "linux" {
                println!("Running on Linux");
                exec_linux(command);
            } else if os == "windows" {
                println!("Running on Windows");
                exec_windows(command);
            } else {
                println!("Running on an unknown operating system");
            }
        }
    }
}
