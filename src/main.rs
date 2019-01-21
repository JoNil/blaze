mod input;
mod vc;

use crate::input::Input;
use crate::vc::v3d;
use crate::vc::Vc;
use std::error::Error;
use std::thread;
use std::time;

fn stop_cursor_blink() {
    
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut vc = Vc::new()?;

    v3d::init()?;

    let mut input = Input::new()?;

    let mut x: i32 = 100;
    let mut y: i32 = 100;

    let mut last = time::Instant::now();
    let mut measure_time = 0;
    let mut average_frame_time = 0.0;

    loop {
        input.poll();

        if input.left_down {
            x -= 1;
        }

        if input.right_down {
            x += 1;
        }

        if input.up_down {
            y -= 1;
        }

        if input.down_down {
            y += 1;
        }

        vc.fb.clear();
        vc.fb.draw(x as u32, y as u32);
        vc.fb.swap();

        let frame_time = {
            let now = time::Instant::now();
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

        thread::sleep(time::Duration::from_millis(10));
    }
}
