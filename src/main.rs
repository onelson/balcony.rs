extern crate opencv;

use std::env;
use opencv::core;
use opencv::highgui;

fn run(url: &str) -> Result<(),String> {
    let window = "video capture";
    try!(highgui::named_window(window,1));
    let mut cam = try!(highgui::VideoCapture::new(url));

    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            try!(highgui::imshow(window, &mut frame));
        }
        if try!(highgui::wait_key(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    let stream_url = env::args().skip(1).next().unwrap();
    println!("{}", stream_url);
    run(&stream_url).unwrap();
}
