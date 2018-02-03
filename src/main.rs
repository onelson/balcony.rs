extern crate opencv;

use std::env;
use opencv::core;
use opencv::highgui;
use opencv::imgproc;

fn run_viewer(url: &str) -> Result<(),String> {
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

fn main2() {
    let stream_url = env::args().skip(1).next().unwrap();
    println!("{}", stream_url);
    run_viewer(&stream_url).unwrap();
}


// ----------------

//import org.opencv.core.Core;
//import org.opencv.core.Core.MinMaxLocResult;
//import org.opencv.core.CvType;
//import org.opencv.core.Mat;
//import org.opencv.core.Point;
//import org.opencv.core.Scalar;
//import org.opencv.highgui.Highgui;
//import org.opencv.imgproc.Imgproc;


struct MinMaxLocResult { min_loc: core::Point, max_loc: core::Point }


fn min_max_loc(mat: &core::Mat) -> MinMaxLocResult {
    unimplemented!();
}


fn run(in_file: &str, template_file: &str, out_file: &str, match_method: i32) -> Result<(), String> {
    println!("Running Template Matching");
    let img: core::Mat = highgui::imread(in_file, 0)?;
    let templ: core::Mat = highgui::imread(template_file, 0)?;

    let tp_size = templ.size()?;

    // / Create the result matrix
    let result: core::Mat = {
        let im_size = img.size()?;
        let result_cols: i32 = im_size.width - tp_size.width + 1;
        let result_rows: i32 = im_size.height - tp_size.height + 1;
        core::Mat::new_rows_cols(result_rows, result_cols, core::CV_32FC1)?
    };

    // / Do the Matching and Normalize
    imgproc::match_template(&img, &templ, &result, match_method)?;
    core::normalize(&result, &result, 0.0, 1.0, core::NORM_MINMAX, -1, &core::Mat::new()?)?;

    // / Localizing the best match with minMaxLoc
    let mmr = min_max_loc(&result);

    let match_loc =
        if match_method == imgproc::TM_SQDIFF || match_method == imgproc::TM_SQDIFF_NORMED {
            mmr.min_loc
        } else {
            mmr.max_loc
        };

    // / Show me what you got
    core::rectangle(
        &img,
        core::Rect {
            x: match_loc.x,
            y: match_loc.y,
            width: tp_size.width,
            height: tp_size.height
        },
        core::Scalar { data: [0.0, 255.0, 0.0, 1.0] },
        1,
        8,
        0
    )?;

    // Save the visualized detection.
    println!("Writing {}", out_file);
    let params = opencv::types::VectorOfint::new();
    highgui::imwrite(out_file, &img, &params)?;

    Ok(())

}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run(&args[0], &args[1], &args[2], imgproc::TM_CCOEFF).unwrap();
}
