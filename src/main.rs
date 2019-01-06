mod vc;

use crate::vc::Vc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut vc = Vc::new()?;

    let mut x: i32 = 100;
    let mut y: i32 = 100;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;

    loop {
        x += dx;
        y += dy;

        if x > 200 {
            dx = -1;
            x = 200;
        }

        if x < 100 {
            dx = 1;
            x = 100;
        }

        if y > 200 {
            dy = -1;
            y = 200;
        }

        if y < 100 {
            dy = 1;
            y = 100;
        }

        vc.fb.draw(x as u32, y as u32);

        vc.fb.swap(&vc.mb)
    }
}
