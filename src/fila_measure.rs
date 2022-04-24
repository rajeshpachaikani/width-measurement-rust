use opencv::{
    prelude::*,
    Result,
    imgproc,
    core,
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
    let slope_out = -1.0/slope;
    let intercept_out = y - slope_out * x;
    (slope_out, intercept_out)
}

//Get Line beginning point
fn line_begins_at(line: &Vec2f) -> (f64, f64){
    let rho = line.0[0];
    let theta = line.0[1];
    let slope = get_slope(rho as f64, theta as f64);
    let c = get_y_intercept(rho as f64, theta as f64);

    let point_1: (f64, f64) = get_point_with_y(slope, c, 0.0);

    return point_1

}

// Split Lines
pub fn split_lines(lines: &Mat) -> (Vec<i32>,Vec<i32>){
    let mut left_indices : Vec<i32> = Vec::new();
    let mut right_indices : Vec<i32> = Vec::new();

    if lines.empty(){
        return (left_indices, right_indices)
    }
    for i in 0..lines.rows(){
        let line = lines.at::<Vec2f>(i).unwrap();
        let point = line_begins_at(line);
        if point.0 < 320.0{
            left_indices.push(i);
        }else{
            right_indices.push(i);
        }
    }
    return (left_indices, right_indices)
}

//Draw Single Line
pub fn draw_line(mut img: &mut Mat, line: &Vec2f){
    let rho = line.0[0];
    let theta = line.0[1];
    let slope = get_slope(rho as f64, theta as f64);
    let c = get_y_intercept(rho as f64, theta as f64);

    let point_1: (f64, f64) = get_point_with_y(slope, c, 0.0);
    let point_2: (f64, f64) = get_point_with_y(slope, c, 480.0);

    imgproc::line(&mut img,
         Point::new(point_1.0 as i32, point_1.1 as i32),
         Point::new(point_2.0 as i32, point_2.1 as i32),
         Scalar::new(0., 0., 255., 0.),
         2,
         imgproc::LINE_AA,
        0
    );
}


// Draw Lines
pub fn draw_lines( mut img : &mut Mat ,lines: &Mat) {
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