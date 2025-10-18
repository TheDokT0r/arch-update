use std::{process::Command, thread};

use spinoff::{Color, Spinner, spinners};

fn main() {
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
