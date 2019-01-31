use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

// Prerequisites
// rustup target add armv7-unknown-linux-gnueabihf
// sudo apt-get install gcc-arm-linux-gnueabihf

fn main() -> Result<(), Box<dyn Error>> {
    env::set_current_dir("../")?;

    Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "--target=armv7-unknown-linux-gnueabihf",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Command::new("robocopy.exe")
        .args(&[
            "/njh", "/njs", "/ndl", "/nc", "/ns",
            "target/armv7-unknown-linux-gnueabihf/release",
            r#"\\192.168.0.35\pi"#,
            "blaze"
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Command::new("ssh")
        .args(&["-t", "pi@192.168.0.35", "sudo", "./blaze"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
