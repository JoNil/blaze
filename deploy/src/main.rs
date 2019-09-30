use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

// Prerequisites
// rustup target add armv7-unknown-linux-gnueabihf
// sudo apt-get install gcc-arm-linux-gnueabihf

fn main() -> Result<(), Box<dyn Error>> {
    let ip = "192.168.0.35";

    env::set_current_dir("../")?;

    {
        let result = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--target=armv7-unknown-linux-gnueabihf",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !result.status.success() {
            return Ok(());
        }
    }

    {
        Command::new("robocopy.exe")
            .args(&[
                "/njh",
                "/njs",
                "/ndl",
                "/nc",
                "/ns",
                "target/armv7-unknown-linux-gnueabihf/release",
                &format!(r#"\\{}\pi"#, ip),
                "blaze",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
    }

    {
        let result = Command::new("ssh")
            .args(&["-t", &format!("pi@{}", ip), "sudo", "./blaze"])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !result.status.success() {
            return Ok(());
        }
    }

    Ok(())
}
