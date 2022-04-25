/*
    Author: Rajesh Pachaikani
    Github: github.com/rajeshpachaikani
    Title: Filament Measurement
    Description: To measure and display the filament width using a digital microscope.
    Calibration values:
        The camera is focused to capture 6mm x 4.5 mm
        The camera resolution is 640x480 in pixels
        Therefore each pixel represents 6/640 or 4.5/480 = 0.009375mm

 */

use opencv::{
    highgui,
    imgproc,
    prelude::*,
    Result,
    videoio,
    core::{Point, Point_, Scalar, Vec2b, Vec2f},
};

mod fila_measure;

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(2, videoio::CAP_ANY)?;

    let mut img = Mat::default();
    loop {
        cam.read(&mut img)?;
        print!("Filament width in mm::{}\n", fila_measure::get_measurement(&mut img));
        highgui::imshow("Filament Measurement", &img)?;
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    Ok(())
}

