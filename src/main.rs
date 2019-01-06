mod vc;

use crate::vc::Vc;
use std::error::Error;
use std::thread;
use std::time;

fn main() -> Result<(), Box<dyn Error>> {
    let mut vc = Vc::new()?;

    let mut x: i32 = 100;
    let mut y: i32 = 100;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;

    let mut last = time::Instant::now();
    let mut measure_time = 0;
    let mut average_frame_time = 0.0;

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

        vc.fb.clear();
        vc.fb.draw(x as u32, y as u32);
        vc.fb.swap(&vc.mb);

        let frame_time = {
            let mut now = time::Instant::now();
            let frame_duration = now.duration_since(last);
            last = now;
            frame_duration.as_secs() as f32 + frame_duration.subsec_nanos() as f32 * 1e-9
        };
        
        measure_time += 1;
        average_frame_time = average_frame_time * 0.95 + frame_time * 0.05;

        if measure_time == 100 {
            measure_time = 0;
            println!("{}", average_frame_time);
        }

        //thread::sleep(time::Duration::from_millis(10));
    }
}
