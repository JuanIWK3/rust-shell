use std::env;
use std::io::{self, Write};
use std::process::Command;

fn cmd(command: &str) {
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command.trim())
        .output()
        .expect("Failed to run!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn bash(mut command: &str) {
    if command == "rm -d" {
        command = "rm -rf";
    } else if command == "rm -a" {
        command = "rm -f";
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(command.trim())
        .output()
        .expect("Failed to run!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let mut linux_editor = Command::new("nano");
    let mut windows_editor = Command::new("notepad");
    let os = env::consts::OS;
    let info = os_info::get();

    if os == "linux" {
        println!("Running on Linux");
    } else if os == "windows" {
        println!("Running on Windows");
    } else {
        println!("Running on an unknown operating system");
    }

    loop {
        let mut command = String::new();

        print!("{}> ", env::current_dir().unwrap().display());

        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line!");

        let command = command.trim();

        if command == "exit" {
            break;
        } else if command == "ver" {
            println!("{}", info);
        }

        if command.starts_with("cd ") {
            let dir = &command[3..];
            match env::set_current_dir(dir) {
                Ok(_) => (),
                Err(e) => println!("Failed to set current dir: {}", e),
            }
        } else if command.starts_with("edit ") {
            let path = &command[5..];

            if os == "linux" {
                linux_editor
                    .arg(path)
                    .spawn()
                    .expect("Failed to open editor");
            } else if os == "windows" {
                windows_editor
                    .arg(path)
                    .spawn()
                    .expect("Failed to open editor");
            } else {
                bash(command);
            }

            let mut editor = Command::new("notepad")
                .arg(path)
                .spawn()
                .expect("Failed to open editor");

            editor.wait().unwrap();
        } else {
            if os == "linux" {
                bash(command);
            } else if os == "windows" {
                cmd(command);
            } else {
                bash(command);
            }
        }
    }
}
