use std::{panic, process::Command, thread};

use spinoff::{Color, Spinner, spinners};

fn main() {
    let avaliable_updates = get_available_arch_updates();
    eprintln!("{:#?}", avaliable_updates);

    let mut spinner = Spinner::new(spinners::Dots, "Downloading updates...", Color::Blue);

    let handle1 = thread::spawn(|| {
        Command::new("sudo")
            .args(["pacman", "-Syyu", "--noconfirm"])
            .output()
            .expect("Failed to update pacman packages");
    });

    let handle2 = thread::spawn(|| {
        Command::new("flatpak")
            .args(["update", "-y"])
            .output()
            .expect("Failed to update flatpak packages");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    spinner.success("Done!");
}

#[derive(Debug)]
struct PacmanPackageData {
    name: String,
    current_ver: String,
    new_ver: String,
}

fn get_available_arch_updates() -> Vec<PacmanPackageData> {
    let output = Command::new("checkupdates")
        .output()
        .expect("Something went wrong while searching for Arch updates");

    if !output.status.success() {
        panic!("Something went wrong while getting available pacman updates");
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(e) => panic!("Command output was not valid UTF-8: {}", e),
    };

    let mut packages = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 3 {
            let package_data = PacmanPackageData {
                name: parts[0].to_string(),
                current_ver: parts[1].to_string(),
                new_ver: parts[2].to_string(),
            };

            packages.push(package_data);
        }
    }

    packages
}
