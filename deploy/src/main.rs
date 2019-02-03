use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

// Prerequisites
// rustup target add armv7-unknown-linux-gnueabihf
// sudo apt-get install gcc-arm-linux-gnueabihf

fn main() -> Result<(), Box<dyn Error>> {

    let ip = "10.0.0.47";
    
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
            &format!(r#"\\{}\pi"#, ip),
            "blaze"
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Command::new("ssh")
        .args(&["-t", &format!("pi@{}", ip), "sudo", "./blaze"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
