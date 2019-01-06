mod vc;

use crate::vc::Vc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let vc = Vc::new();

    Ok(())
}
