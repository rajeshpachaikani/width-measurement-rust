/*
    Author: Rajesh Pachaikani
    Github: github.com/rajeshpachaikani
    Title: Filament Measurement
    Description: To measure and display the filament width using a digital microsope.
    Calibration values:
        The camera is focused to capture 6mm x 4.5 mm
        The camera resolution is 640x480 in pixels
        Therefore each pixel represents 6/640 or 4.5/480 = 0.009375mm

 */

use opencv::{
    highgui,
    videoio,
    imgproc,
    Result,
    prelude::*,
};
use opencv::core::{Point, Point_, Scalar, Vec2b, Vec2f};

mod fila_measure;

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(2, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut canny = Mat::default();
    loop {
        cam.read(&mut img)?;
        imgproc::canny(
            &img,
            &mut canny,
            100.0,
            200.0,
            3,
            false,
        )?;

        // Find Hough Lines
        let mut lines = Mat::default();
        imgproc::hough_lines(
            &canny,
            &mut lines,
            1.0,
            std::f64::consts::PI / 180.0,
            100,
            0.0,
            0.0,
            0.0,
            std::f64::consts::PI,
        )?;

        // fila_measure::draw_lines(&mut img, &lines);
        if !lines.empty() {
            let left_right = fila_measure::split_lines(&lines);
            //println!("Left Index{:?}", left_right.0);
            // println!("Right Index{:?}", left_right.1);
            if !left_right.0.is_empty() && !left_right.1.is_empty() {
                let line_left = lines.at::<Vec2f>(left_right.0[0])?;
                let line_right = lines.at::<Vec2f>(left_right.1[0])?;
                fila_measure::draw_line(&mut img, &line_left);
                fila_measure::draw_line(&mut img, &line_right);

                //Slope and intercept of left line
                let slope_left = fila_measure::get_slope(line_left.0[0] as f64, line_left.0[1] as f64);
                let intercept_left = fila_measure::get_y_intercept(line_left.0[0] as f64, line_left.0[1] as f64);

                //Slope and intercept of right line
                let slope_right = fila_measure::get_slope(line_right.0[0] as f64, line_right.0[1] as f64);
                let intercept_right = fila_measure::get_y_intercept(line_right.0[0] as f64, line_right.0[1] as f64);

                // Get point at y = 240
                let point_left = fila_measure::get_point_with_y(slope_left, intercept_left, 240.0);

                // Get Perpendicular line
                let normal_line = fila_measure::get_normal(point_left.0, point_left.1, slope_left, intercept_left);

                // Get Intersection point
                let intersection_point = fila_measure::get_intersection_point(normal_line.0, normal_line.1, slope_right, intercept_right);

                let point_l = Point::new(point_left.0 as i32, point_left.1 as i32);
                let point_r = Point::new(intersection_point.0 as i32, intersection_point.1 as i32);

                // println!("{:?}", intersection_point);
                imgproc::line(
                    &mut img,
                    Point::new(point_left.0 as i32, point_left.1 as i32),
                    Point::new(intersection_point.0 as i32, intersection_point.1 as i32),
                    Scalar::new(0.0, 255.0, 0.0, 0.0),
                    1,
                    imgproc::LINE_AA,
                    0,
                )?;
                imgproc::circle(
                    &mut img,
                    Point::new(intersection_point.0 as i32, intersection_point.1 as i32),
                    5,
                    Scalar::new(255., 0., 255., 0.),
                    5,
                    imgproc::LINE_AA,
                    0,
                )?;
                let dist = fila_measure::euclidean_distance(&point_l, &point_r);
                println!("Filament Width in mm: {:?}", dist*0.009375);
                // println!("{:?}", fila_measure::euclidean_distance(&point_l, &point_r));
            }

            // println!("{:?}", dist);

        }
        highgui::imshow("Canny", &canny)?;
        highgui::imshow("Filament Measurement", &img)?;
        let key = highgui::wait_key(1)?;
        if key == 27 || key == 'q' as i32 {
            break;
        }
    }


    Ok(())
}

