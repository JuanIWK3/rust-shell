use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

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
            let mut linux_editor = Command::new("nano");
            let mut windows_editor = Command::new("notepad");
            let path = &command[5..];

            let lock_file = format!("{}.lock", path);

            if Path::new(&lock_file).exists() {
                println!("The file is already being edited by another process.");
                continue;
            }

            fs::File::create(&lock_file).expect("Failed to create lock file");

            if os == "windows" {
                println!("{}", path);
                let mut editor = windows_editor
                    .arg(path)
                    .spawn()
                    .expect("Failed to open editor");

                editor.wait().unwrap();
            } else {
                let mut editor = linux_editor
                    .arg(path)
                    .spawn()
                    .expect("Failed to open editor");

                editor.wait().unwrap();
            }

            fs::remove_file(&lock_file).expect("Failed to remove lock file")
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
