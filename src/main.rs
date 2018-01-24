extern crate opencv;

use std::env;
use opencv::core;
use opencv::highgui;

fn run(url: &str) -> Result<(),String> {
    let mut cam = highgui::VideoCapture::new(url)?;

    if !cam.is_opened()? {
        Err(format!("unable to open: `{}`", url))
    } else {
        let window = "video capture";
        highgui::named_window(window, 1)?;

        loop {
            let mut frame = core::Mat::new()?;

            cam.read(&mut frame)?;

            if frame.size()?.width > 0 {
                highgui::imshow(window, &mut frame)?;
            }

            if highgui::wait_key(10)? > 0 {
                return Ok(())
            }
        }
    }
}

fn main() {
    let stream_url = env::args().skip(1).next().unwrap();
    println!("{}", stream_url);
    run(&stream_url).unwrap();
}
