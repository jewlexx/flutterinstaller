use dirs::home_dir;
use git2::Repository;
use spinners_rs::{Spinner, Spinners};
use std::{
    fs::create_dir_all,
    io::{stdout, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

#[cfg(target_os = "windows")]
const FILE_EXTENSION: &str = ".bat";
#[cfg(not(target_os = "windows"))]
const FILE_EXTENSION: &str = "";

fn main() {
    let mut flutter_dir = home_dir().unwrap();
    flutter_dir.push("Tools");
    create_dir_all(&flutter_dir).expect("Failed to create dirs");
    flutter_dir.push("flutter");

    let sp = Spinner::new(&Spinners::Dots, "Cloning flutter repo...".into());

    match Repository::clone("https://github.com/flutter/flutter.git", &flutter_dir) {
        Ok(repo) => {
            sp.stop();
            print!("\n");
            repo
        }
        Err(_) => {
            sp.stop();
            print!("\n");
            println!("Failed to clone flutter repo");
            println!("Check if the directory already exists");
            return;
        }
    };

    flutter_dir.push("bin");
    flutter_dir.push(format!("flutter{}", FILE_EXTENSION));

    Command::new(&flutter_dir).arg("doctor");
}
