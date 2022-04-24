/*
    Author: Rajesh Pachaikani
    Date: 2022-04-19
    Title: Filament Width Measurement
    Description: Program to measure filament width and serve the value over websocket
 */


use std::any::Any;
use std::f64::consts::PI;
use opencv::prelude::*;
use opencv::{
    highgui,
    videoio,
    imgproc,
    Result,

    core::{
        Point_,
        Rect,
        Scalar,
        Size
    },
};
use opencv::imgproc::{canny, hough_lines_p, line, threshold};

//Empty callback function
fn empty_callback(_: &str) {
}

fn main() -> Result<()>{
    let mut camera = videoio::VideoCapture::new(2,videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut left_roi = Mat::default();
    let mut right_roi = Mat::default();
    let mut left_bin = Mat::default();
    let mut right_bin = Mat::default();
    let mut cloned_img = img.clone();
    loop {
        camera.read(&mut img)?;
        highgui::imshow("w", &img)?;
        let left_rect = Rect::from_points(Point_::new(120,50),Point_::new(120+120,50+360));
        let right_rect = Rect::from_points(Point_::new(300,50),Point_::new(300+120,50+360));

        left_roi = Mat::roi(&img, left_rect)?;
        right_roi = Mat::roi(&img, right_rect)?;


        //Draw Rectangle

        imgproc::rectangle(
            &mut img,
            left_rect,
            Scalar::new(1.,1.,1.,1.),
            2,
            imgproc::LINE_4,
            0)?;

        imgproc::rectangle(
            &mut img,
            right_rect,
            Scalar::new(255.,1.,1.,1.),
            2,
            imgproc::LINE_4,
            0
        )?;

        let mut lines = Mat::default();
        canny(&left_roi, &mut left_bin, 1.,100.,3,true)?;
        canny(&right_roi, &mut right_bin, 1.,100.,3,true)?;


        highgui::imshow("left", &left_bin)?;
        highgui::imshow("right", &right_bin)?;
        let key = highgui::wait_key(1)?;
        if key == ('q' as u32) as i32 || key == 27{
            println!("Bye Bye");
            break;
        }
    }

    Ok(())
}
