use std::process::{Command, Stdio};
use std::error::Error;
use std::env;

// Prerequisites
// rustup target add armv7-unknown-linux-gnueabihf
// sudo apt-get install gcc-arm-linux-gnueabihf

fn main() -> Result<(), Box<dyn Error>> {
    
    env::set_current_dir("../")?;

    Command::new("cargo")
        .args(&["build", "--release", "--target=armv7-unknown-linux-gnueabihf"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Command::new("scp")
        .args(&["target/armv7-unknown-linux-gnueabihf/release/rpi_gpu_game", "pi@192.168.0.35:~/"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Command::new("ssh")
        .args(&["-t", "pi@192.168.0.35", "sudo", "./rpi_gpu_game"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
