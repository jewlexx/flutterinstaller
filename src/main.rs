use dirs::home_dir;
use git2::Repository;
use spinner::SpinnerBuilder;
use std::fs::create_dir_all;

fn main() {
    let mut flutter_dir = home_dir().unwrap();
    flutter_dir.push("Tools");
    create_dir_all(&flutter_dir).expect("Failed to create dirs");
    flutter_dir.push("flutter");

    let sp = SpinnerBuilder::new("Cloning flutter repo...".into()).start();

    Repository::clone("https://github.com/flutter/flutter.git", flutter_dir)
        .expect("Failed to clone flutter");

    sp.close();
}
