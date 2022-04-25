use opencv::{
    core,
    imgproc,
    prelude::*,
    Result,
};
use opencv::core::{Point, Scalar, Vec2b, Vec2f};

pub fn get_point_with_y(slope: f64, c: f64, y: f64) -> (f64, f64) {
    ((y - c) / slope, y)
}

// Get Intersection Point
pub fn get_intersection_point(slope1: f64, c1: f64, slope2: f64, c2: f64) -> (f64, f64) {
    let x = (c2 - c1) / (slope1 - slope2);
    let y = slope1 * x + c1;
    (x, y)
}

pub fn get_y_intercept(rho: f64, theta: f64) -> f64 {
    rho / theta.sin()
}

pub fn get_slope(rho: f64, theta: f64) -> f64 {
    -(theta.cos() / theta.sin())
}


// Euclidean Distance
pub fn euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let x = (p1.x - p2.x) as f64;
    let y = (p1.y - p2.y) as f64;
    ((x * x + y * y) as f64).sqrt()
}

pub fn get_normal(x: f64, y: f64, slope: f64, c: f64) -> (f64, f64) {
    let slope_out = -1.0 / slope;
    let intercept_out = y - slope_out * x;
    (slope_out, intercept_out)
}

//Get Line beginning point
fn line_begins_at(line: &Vec2f) -> (f64, f64) {
    let rho = line.0[0];
    let theta = line.0[1];
    let slope = get_slope(rho as f64, theta as f64);
    let c = get_y_intercept(rho as f64, theta as f64);

    let point_1: (f64, f64) = get_point_with_y(slope, c, 0.0);

    return point_1;
}

// Split Lines
pub fn split_lines(lines: &Mat) -> (Vec<u8>, Vec<u8>) {
    let mut left_indices: Vec<u8> = Vec::new();
    let mut right_indices: Vec<u8> = Vec::new();

    if lines.empty() {
        return (left_indices, right_indices);
    }
    for i in 0..lines.rows() {
        let line = lines.at::<Vec2f>(i).unwrap();
        let point = line_begins_at(line);
        if point.0 < 320.0 && point.0 > 20.0 {
            left_indices.push(i as u8);
        } else if point.0 > 320.0 && point.0 < 620.0 {
            right_indices.push(i as u8);
        }
    }
    return (left_indices, right_indices);
}

//Draw Single Line
pub fn draw_line(mut img: &mut Mat, line: &Vec2f, color: &Scalar) {
    let rho = line.0[0];
    let theta = line.0[1];
    let slope = get_slope(rho as f64, theta as f64);
    let c = get_y_intercept(rho as f64, theta as f64);

    let point_1: (f64, f64) = get_point_with_y(slope, c, 0.0);
    let point_2: (f64, f64) = get_point_with_y(slope, c, 480.0);

    imgproc::line(&mut img,
                  Point::new(point_1.0 as i32, point_1.1 as i32),
                  Point::new(point_2.0 as i32, point_2.1 as i32),
                  *color,
                  2,
                  imgproc::LINE_AA,
                  0,
    );
}


// Draw Lines
pub fn draw_lines(mut img: &mut Mat, lines: &Mat) {
    if lines.rows() > 0 {
        for i in 0..lines.rows() {
            let rho_theta = lines.at::<Vec2f>(i).unwrap();
            let rho = rho_theta.0[0];
            let theta = rho_theta.0[1];
            let slope = get_slope(rho as f64, theta as f64);
            let c = get_y_intercept(rho as f64, theta as f64);

            let point_1: (f64, f64) = get_point_with_y(slope, c, 0.0);
            let point_2: (f64, f64) = get_point_with_y(slope, c, 480.0);

            imgproc::line(
                &mut img,
                Point::new(point_1.0 as i32, point_1.1 as i32),
                Point::new(point_2.0 as i32, point_2.1 as i32),
                Scalar::new(0.0, 255.0, 0.0, 255.0),
                2,
                imgproc::LINE_AA,
                0,
            ).unwrap();
        }
    }
}

// Get Measurement from image
pub fn get_measurement(mut img: &mut Mat) -> f64 {
    let mut canny = Mat::default();
    let mut image = img.clone();
    let mut result: f64 = 0.0;
    imgproc::canny(
        &image,
        &mut canny,
        140.0,
        200.0,
        3,
        false,
    ).unwrap();

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
    ).unwrap();

    // draw_lines(&mut img, &lines);
    if !lines.empty() {
        let left_right = split_lines(&lines);
        //println!("Left Index{:?}", left_right.0);
        // println!("Right Index{:?}", left_right.1);
        if !left_right.0.is_empty() && !left_right.1.is_empty() {
            let line_left = lines.at::<Vec2f>(left_right.0[0] as i32).unwrap();

            let line_right = lines.at::<Vec2f>(left_right.1[0] as i32).unwrap();
            let left_color = Scalar::new(0.0, 255.0, 0.0, 255.0);
            let right_color = Scalar::new(0.0, 0.0, 255.0, 255.0);
            draw_line(&mut img, &line_left, &left_color);
            draw_line(&mut img, &line_right, &right_color);

            //Slope and intercept of left line
            let slope_left = get_slope(line_left.0[0] as f64, line_left.0[1] as f64);
            let intercept_left = get_y_intercept(line_left.0[0] as f64, line_left.0[1] as f64);

            //Slope and intercept of right line
            let slope_right = get_slope(line_right.0[0] as f64, line_right.0[1] as f64);
            let intercept_right = get_y_intercept(line_right.0[0] as f64, line_right.0[1] as f64);

            // Get point at y = 240
            let point_left = get_point_with_y(slope_left, intercept_left, 240.0);

            // Get Perpendicular line
            let normal_line = get_normal(point_left.0, point_left.1, slope_left, intercept_left);

            // Get Intersection point
            let intersection_point = get_intersection_point(normal_line.0, normal_line.1, slope_right, intercept_right);

            let point_l = Point::new(point_left.0 as i32, point_left.1 as i32);
            let point_r = Point::new(intersection_point.0 as i32, intersection_point.1 as i32);

            let dist = euclidean_distance(&point_l, &point_r);
            // println!("Filament Width in mm: {:?}", dist * 0.009375);
            result = dist * 0.009375;
        }
    }
    result
}

